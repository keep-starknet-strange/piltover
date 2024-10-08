//! SPDX-License-Identifier: MIT
//!
//!

mod errors {
    const INVALID_ADDRESS: felt252 = 'Config: invalid address';
    const SNOS_INVALID_PROGRAM_OUTPUT_SIZE: felt252 = 'snos: invalid output size';
    const SNOS_INVALID_CONFIG_HASH: felt252 = 'snos: invalid config hash';
    const SNOS_INVALID_MESSAGES_SEGMENTS: felt252 = 'snos: invalid messages segments';
    const NO_STATE_TRANSITION_PROOF: felt252 = 'no state transition proof';
}

/// Appchain settlement contract on starknet.
#[starknet::contract]
mod appchain {
    use openzeppelin::access::ownable::{
        OwnableComponent as ownable_cpt, OwnableComponent::InternalTrait as OwnableInternal,
        interface::IOwnable
    };
    use openzeppelin::security::reentrancyguard::{
        ReentrancyGuardComponent,
        ReentrancyGuardComponent::InternalTrait as InternalReentrancyGuardImpl
    };
    use openzeppelin::upgrades::{
        UpgradeableComponent as upgradeable_cpt,
        UpgradeableComponent::InternalTrait as UpgradeableInternal, interface::IUpgradeable
    };
    use piltover::components::onchain_data_fact_tree_encoder::{
        encode_fact_with_onchain_data, DataAvailabilityFact
    };
    use piltover::config::{config_cpt, config_cpt::InternalTrait as ConfigInternal, IConfig};
    use piltover::interface::IAppchain;
    use piltover::messaging::{
        messaging_cpt, messaging_cpt::InternalTrait as MessagingInternal, IMessaging,
        output_process, output_process::{MessageToStarknet, MessageToAppchain},
    };
    use piltover::fact_registry::{IFactRegistryDispatcher, IFactRegistryDispatcherTrait};
    use piltover::snos_output::StarknetOsOutput;
    use piltover::snos_output;
    use piltover::state::component::state_cpt::HasComponent;
    use piltover::state::{state_cpt, state_cpt::InternalTrait as StateInternal, IState};
    use core::poseidon::{Poseidon, PoseidonImpl, HashStateImpl, poseidon_hash_span};
    use starknet::{ContractAddress, ClassHash};
    use super::errors;

    /// The default cancellation delay of 5 days.
    const CANCELLATION_DELAY_SECS: u64 = 432000;

    component!(path: ownable_cpt, storage: ownable, event: OwnableEvent);
    component!(path: upgradeable_cpt, storage: upgradeable, event: UpgradeableEvent);
    component!(path: config_cpt, storage: config, event: ConfigEvent);
    component!(path: messaging_cpt, storage: messaging, event: MessagingEvent);
    component!(path: state_cpt, storage: state, event: StateEvent);
    component!(
        path: ReentrancyGuardComponent, storage: reentrancy_guard, event: ReentrancyGuardEvent
    );

    #[abi(embed_v0)]
    impl ConfigImpl = config_cpt::ConfigImpl<ContractState>;
    #[abi(embed_v0)]
    impl MessagingImpl = messaging_cpt::MessagingImpl<ContractState>;
    #[abi(embed_v0)]
    impl StateImpl = state_cpt::StateImpl<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        ownable: ownable_cpt::Storage,
        #[substorage(v0)]
        upgradeable: upgradeable_cpt::Storage,
        #[substorage(v0)]
        config: config_cpt::Storage,
        #[substorage(v0)]
        messaging: messaging_cpt::Storage,
        #[substorage(v0)]
        reentrancy_guard: ReentrancyGuardComponent::Storage,
        #[substorage(v0)]
        state: state_cpt::Storage,
        messages_to_appchain: Array<felt252>,
        messages_to_starknet: Array<felt252>,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        OwnableEvent: ownable_cpt::Event,
        #[flat]
        UpgradeableEvent: upgradeable_cpt::Event,
        #[flat]
        ConfigEvent: config_cpt::Event,
        #[flat]
        MessagingEvent: messaging_cpt::Event,
        #[flat]
        ReentrancyGuardEvent: ReentrancyGuardComponent::Event,
        #[flat]
        StateEvent: state_cpt::Event,
        LogStateUpdate: LogStateUpdate,
        LogStateTransitionFact: LogStateTransitionFact,
    }

    #[derive(Drop, starknet::Event)]
    struct LogStateUpdate {
        state_root: felt252,
        block_number: felt252,
        block_hash: felt252,
    }

    #[derive(Drop, starknet::Event)]
    struct LogStateTransitionFact {
        state_transition_fact: u256,
    }

    /// Initializes the contract.
    ///
    /// # Arguments
    ///
    /// * `address` - The contract address of the owner.
    #[constructor]
    fn constructor(
        ref self: ContractState,
        owner: ContractAddress,
        state_root: felt252,
        block_number: felt252,
        block_hash: felt252,
    ) {
        self.ownable.initializer(owner);
        self.messaging.initialize(CANCELLATION_DELAY_SECS);
        self.state.initialize(state_root, block_number, block_hash);
    }


    #[abi(embed_v0)]
    impl Appchain of IAppchain<ContractState> {
        fn update_state(
            ref self: ContractState,
            program_output: Span<felt252>,
            onchain_data_hash: felt252,
            onchain_data_size: u256
        ) {
            self.reentrancy_guard.start();
            self.config.assert_only_owner_or_operator();
            let output_hash = poseidon_hash_span(program_output);

            // Header size + 2 messages segments len.
            assert(
                program_output.len() > snos_output::HEADER_SIZE + 2,
                errors::SNOS_INVALID_PROGRAM_OUTPUT_SIZE
            );

            let (current_program_hash, current_config_hash): (felt252, felt252) = self
                .config
                .program_info
                .read();

            let data_availability_fact: DataAvailabilityFact = DataAvailabilityFact {
                onchain_data_hash, onchain_data_size
            };
            let state_transition_fact: u256 = encode_fact_with_onchain_data(
                program_output, data_availability_fact
            );

            let mut stripped_output = program_output.slice(1, program_output.len() / 2);
            let program_output_struct: StarknetOsOutput = Serde::deserialize(ref stripped_output)
                .unwrap();
            assert(
                program_output_struct.os_program_hash == current_config_hash,
                errors::SNOS_INVALID_CONFIG_HASH
            );

            let fact = poseidon_hash_span(array![current_program_hash, output_hash].span());

            assert(
                IFactRegistryDispatcher { contract_address: self.config.get_facts_registry() }
                    .is_valid(fact),
                errors::NO_STATE_TRANSITION_PROOF
            );

            self.emit(LogStateTransitionFact { state_transition_fact });

            // Perform state update
            self.state.update(program_output);

            // TODO(#7): We should update SNOS output to have the messages count
            // instead of the messages segment len.
            let mut starknet_segment = program_output_struct.messages_to_l1.span();
            
            if starknet_segment.is_empty() {
                starknet_segment = [0].span();
            }
            let messages_to_starknet = Serde::<Array<MessageToStarknet>>::deserialize(ref starknet_segment)
                .expect('bad format message sn');

            let mut appchain_segment = program_output_struct.messages_to_l2.span();
            if appchain_segment.is_empty() {
                appchain_segment = [0].span();
            }
            let messages_to_appchain = Serde::<
                Array<MessageToAppchain>
            >::deserialize(ref appchain_segment)
                .expect('bad format message sn');

            self.messaging.process_messages_to_starknet(messages_to_starknet.span());
            self.messaging.process_messages_to_appchain(messages_to_appchain.span());
            self.reentrancy_guard.end();

            self
                .emit(
                    LogStateUpdate {
                        state_root: self.state.state_root.read(),
                        block_number: self.state.block_number.read(),
                        block_hash: self.state.block_hash.read(),
                    }
                );
        }
    }

    #[abi(embed_v0)]
    impl UpgradeableImpl of IUpgradeable<ContractState> {
        fn upgrade(ref self: ContractState, new_class_hash: ClassHash) {
            self.ownable.assert_only_owner();
            self.upgradeable.upgrade(new_class_hash);
        }
    }
}
