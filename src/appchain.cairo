//! SPDX-License-Identifier: MIT
//!
//!

mod errors {
    pub const INVALID_ADDRESS: felt252 = 'Config: invalid address';
    pub const SNOS_INVALID_PROGRAM_OUTPUT_SIZE: felt252 = 'snos: invalid output size';
    pub const SNOS_INVALID_OUTPUT_HASH: felt252 = 'snos: invalid output hash';
    pub const SNOS_INVALID_PROGRAM_HASH: felt252 = 'snos: invalid program hash';
    pub const SNOS_INVALID_CONFIG_HASH: felt252 = 'snos: invalid config hash';
    pub const SNOS_INVALID_MESSAGES_SEGMENTS: felt252 = 'snos: invalid messages segments';
    pub const NO_STATE_TRANSITION_PROOF: felt252 = 'no state transition proof';
    pub const NO_FACT_REGISTERED: felt252 = 'no fact registered';
    pub const LAYOUT_BRIDGE_INVALID_PROGRAM_HASH: felt252 = 'lb: invalid program hash';
}

/// Appchain settlement contract on starknet.
#[starknet::contract]
pub mod appchain {
    use core::iter::IntoIterator;
    use core::poseidon::{PoseidonImpl, poseidon_hash_span};
    use openzeppelin::access::ownable::{
        OwnableComponent as ownable_cpt, OwnableComponent::InternalTrait as OwnableInternal,
    };
    use openzeppelin::security::reentrancyguard::{
        ReentrancyGuardComponent,
        ReentrancyGuardComponent::InternalTrait as InternalReentrancyGuardImpl,
    };
    use openzeppelin::upgrades::{
        UpgradeableComponent as upgradeable_cpt,
        UpgradeableComponent::InternalTrait as UpgradeableInternal, interface::IUpgradeable,
    };
    use piltover::components::onchain_data_fact_tree_encoder::{
        DataAvailabilityFact, encode_fact_with_onchain_data,
    };
    use piltover::config::{IConfig, config_cpt, config_cpt::InternalTrait as ConfigInternal};
    use piltover::fact_registry::{IFactRegistryDispatcher, IFactRegistryDispatcherTrait};
    use piltover::interface::IAppchain;
    use piltover::messaging::{messaging_cpt, messaging_cpt::InternalTrait as MessagingInternal};
    use piltover::snos_output::deserialize_os_output;
    use piltover::state::{IState, state_cpt, state_cpt::InternalTrait as StateInternal};
    use starknet::storage::{StoragePointerReadAccess};
    use starknet::{ClassHash, ContractAddress};
    use super::errors;

    /// The default cancellation delay of 5 days.
    const CANCELLATION_DELAY_SECS: u64 = 432000;

    component!(path: ownable_cpt, storage: ownable, event: OwnableEvent);
    component!(path: upgradeable_cpt, storage: upgradeable, event: UpgradeableEvent);
    component!(path: config_cpt, storage: config, event: ConfigEvent);
    component!(path: messaging_cpt, storage: messaging, event: MessagingEvent);
    component!(path: state_cpt, storage: state, event: StateEvent);
    component!(
        path: ReentrancyGuardComponent, storage: reentrancy_guard, event: ReentrancyGuardEvent,
    );

    #[abi(embed_v0)]
    impl ConfigImpl = config_cpt::ConfigImpl<ContractState>;
    #[abi(embed_v0)]
    impl MessagingImpl = messaging_cpt::MessagingImpl<ContractState>;
    #[abi(embed_v0)]
    impl StateImpl = state_cpt::StateImpl<ContractState>;

    #[abi(embed_v0)]
    impl OwnableImpl = ownable_cpt::OwnableTwoStepImpl<ContractState>;

    #[cfg(feature: 'messaging_test')]
    #[abi(embed_v0)]
    impl MessagingTestImpl =
        messaging_cpt::MessagingTestImpl<ContractState>;

    #[cfg(feature: 'update_state_test')]
    #[abi(embed_v0)]
    impl StateTestImpl =
        state_cpt::StateTestImpl<ContractState>;

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
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {
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
    pub struct LogStateUpdate {
        pub state_root: felt252,
        pub block_number: felt252,
        pub block_hash: felt252,
    }

    #[derive(Drop, starknet::Event)]
    pub struct LogStateTransitionFact {
        pub state_transition_fact: u256,
    }

    /// Initializes the contract.
    ///
    /// # Arguments
    ///
    /// * `address` - The contract address of the owner.
    /// * `state_root` - The state root of the contract.
    /// * `block_number` - The block number of the contract.
    /// * `block_hash` - The block hash of the contract.
    #[constructor]
    fn constructor(ref self: ContractState, owner: ContractAddress) {
        self.ownable.initializer(owner);
        self.messaging.initialize(CANCELLATION_DELAY_SECS);
        // The prev block number for 0th block in the snos
        //  as "0x800000000000011000000000000000000000000000000000000000000000000"
        // which is 3618502788666131213697322783095070105623107215331596699973092056135872020480 in
        // felt252
        let initial_block_number: felt252 =
            3618502788666131213697322783095070105623107215331596699973092056135872020480;
        self.state.initialize(0, initial_block_number, 0);
    }

    #[abi(embed_v0)]
    impl Appchain of IAppchain<ContractState> {
        fn update_state(
            ref self: ContractState,
            snos_output: Span<felt252>,
            layout_bridge_output: Span<felt252>,
            onchain_data_hash: felt252,
            onchain_data_size: u256,
        ) {
            self.reentrancy_guard.start();
            self.config.assert_only_owner_or_operator();

            let program_info = self.config.program_info.read();

            // StarknetOS (SNOS) proof is wrapped in bootloader so 3rd element is the program hash
            // of bootloaded program, in our case SNOS.
            let snos_program_hash = snos_output.at(2);
            assert(
                program_info.snos_program_hash == *snos_program_hash,
                errors::SNOS_INVALID_PROGRAM_HASH,
            );

            // Layout bridge program is also bootloaded, and the 3rd element is the hash of the
            // output of the program that has been bootloaded.
            let layout_bridge_program_hash = layout_bridge_output.at(2);
            assert(
                program_info.layout_bridge_program_hash == *layout_bridge_program_hash,
                errors::LAYOUT_BRIDGE_INVALID_PROGRAM_HASH,
            );

            let snos_output_hash = poseidon_hash_span(snos_output);
            // Layout bridge program is also bootloaded, and the 5th element is the hash of the
            // output of the program that has been layout-bridged.
            let snos_output_hash_in_bridge_output = layout_bridge_output.at(4);
            assert(
                snos_output_hash == *snos_output_hash_in_bridge_output,
                errors::SNOS_INVALID_OUTPUT_HASH,
            );

            let output_hash = poseidon_hash_span(layout_bridge_output);

            let mut snos_output_iter = snos_output.into_iter();
            let program_output_struct = deserialize_os_output(ref snos_output_iter);

            let data_availability_fact: DataAvailabilityFact = DataAvailabilityFact {
                onchain_data_hash, onchain_data_size,
            };
            let state_transition_fact: u256 = encode_fact_with_onchain_data(
                layout_bridge_output, data_availability_fact,
            );

            assert(
                program_output_struct.starknet_os_config_hash == program_info.snos_config_hash,
                errors::SNOS_INVALID_CONFIG_HASH,
            );

            let fact = poseidon_hash_span(
                array![program_info.bootloader_program_hash, output_hash].span(),
            );
            let verifications = IFactRegistryDispatcher {
                contract_address: self.config.get_facts_registry(),
            }
                .get_all_verifications_for_fact_hash(fact);

            if verifications.len() == 0 {
                core::panic_with_felt252(errors::NO_FACT_REGISTERED)
            };

            assert!(*verifications.at(0).security_bits > 50);

            self.emit(LogStateTransitionFact { state_transition_fact });

            let messages_to_l1 = program_output_struct.messages_to_l1;
            let messages_to_l2 = program_output_struct.messages_to_l2;

            // Perform state update
            self.state.update(program_output_struct);

            self.messaging.process_messages_to_starknet(messages_to_l1);
            self.messaging.process_messages_to_appchain(messages_to_l2);

            self.reentrancy_guard.end();

            self
                .emit(
                    LogStateUpdate {
                        state_root: self.state.state_root.read(),
                        block_number: self.state.block_number.read(),
                        block_hash: self.state.block_hash.read(),
                    },
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
