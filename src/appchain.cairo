//! SPDX-License-Identifier: MIT
//!
//!

mod errors {
    pub const INVALID_ADDRESS: felt252 = 'Config: invalid address';
    pub const SNOS_INVALID_PROGRAM_OUTPUT_SIZE: felt252 = 'snos: invalid output size';
    pub const SNOS_INVALID_PROGRAM_HASH: felt252 = 'snos: invalid program hash';
    pub const SNOS_INVALID_CONFIG_HASH: felt252 = 'snos: invalid config hash';
    pub const SNOS_INVALID_MESSAGES_SEGMENTS: felt252 = 'snos: invalid messages segments';
    pub const NO_STATE_TRANSITION_PROOF: felt252 = 'no state transition proof';
    pub const NO_L1_FACT_ATTESTED: felt252 = 'no l1 fact attested';
    pub const L1_PROGRAM_HASH_MISMATCH: felt252 = 'l1 program hash mismatch';
    pub const L1_STATE_FACT_MISMATCH: felt252 = 'l1 state fact mismatch';
    pub const L1_SHARP_FACT_MISMATCH: felt252 = 'l1 sharp fact mismatch';
    pub const NO_FACT_REGISTERED: felt252 = 'no fact registered';
}

/// Appchain settlement contract on starknet.
#[starknet::contract]
pub mod appchain {
    use core::iter::IntoIterator;
    use openzeppelin::access::ownable::OwnableComponent as ownable_cpt;
    use openzeppelin::access::ownable::OwnableComponent::InternalTrait as OwnableInternal;
    use openzeppelin::security::reentrancyguard::ReentrancyGuardComponent;
    use openzeppelin::security::reentrancyguard::ReentrancyGuardComponent::InternalTrait as InternalReentrancyGuardImpl;
    use openzeppelin::upgrades::UpgradeableComponent as upgradeable_cpt;
    use openzeppelin::upgrades::UpgradeableComponent::InternalTrait as UpgradeableInternal;
    use openzeppelin::upgrades::interface::IUpgradeable;
    use piltover::config::config_cpt::InternalTrait as ConfigInternal;
    use piltover::config::{IConfig, config_cpt};
    use piltover::fact_registry_proxy::{
        IFactRegistryProxyDispatcher, IFactRegistryProxyDispatcherTrait,
    };
    use piltover::interface::IAppchain;
    use piltover::messaging::messaging_cpt;
    use piltover::messaging::messaging_cpt::InternalTrait as MessagingInternal;
    use piltover::snos_output::deserialize_os_output;
    use piltover::state::state_cpt::InternalTrait as StateInternal;
    use piltover::state::{IStateUpdater, state_cpt};
    use starknet::storage::StoragePointerReadAccess;
    use starknet::{ClassHash, ContractAddress};
    use super::errors;

    /// The default cancellation delay of 5 days.
    const CANCELLATION_DELAY_SECS: u64 = 432000;
    const MIN_SECURITY_BITS: u32 = 50;

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
        pub update_id: felt252,
        pub state_transition_fact: u256,
        pub sharp_fact: u256,
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
            snos_output: Span<felt252>,
            layout_bridge_output: Span<felt252>,
        ) {
            let _ = layout_bridge_output;
            self.reentrancy_guard.start();
            self.config.assert_only_owner_or_operator();

            let program_info = self.config.program_info.read();

            let mut bootloaded_snos_output: Array<felt252> = ArrayTrait::new();
            bootloaded_snos_output.append(1);
            bootloaded_snos_output.append((snos_output.len() + 2).into());
            bootloaded_snos_output.append(program_info.snos_program_hash);
            let mut i = 0;
            loop {
                if (i == snos_output.len()) {
                    break;
                }
                bootloaded_snos_output.append(*snos_output.at(i));
                i += 1;
            }

            let mut snos_output_iter = bootloaded_snos_output.span().into_iter();
            let program_output_struct = deserialize_os_output(
                ref snos_output_iter, self.config.get_use_kzg_da(),
            );

            let state_transition_fact: u256 = hash_main_public_input_solidity(snos_output);

            assert(
                program_output_struct.starknet_os_config_hash == program_info.snos_config_hash,
                errors::SNOS_INVALID_CONFIG_HASH,
            );

            let update_id = program_output_struct.prev_block_number;
            let expected_sharp_fact = compute_sharp_fact(
                program_info.snos_program_hash.into(), state_transition_fact,
            );

            let facts_registry = IFactRegistryProxyDispatcher {
                contract_address: self.config.get_facts_registry(),
            };
            assert(
                facts_registry
                    .is_fact_hash_valid_with_security(expected_sharp_fact, MIN_SECURITY_BITS),
                errors::NO_FACT_REGISTERED,
            );

            self
                .emit(
                    LogStateTransitionFact {
                        update_id, state_transition_fact, sharp_fact: expected_sharp_fact,
                    },
                );

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

    fn compute_sharp_fact(fact_program_hash: u256, state_transition_fact: u256) -> u256 {
        let mut keccak_input: Array<u256> = ArrayTrait::new();
        keccak_input.append(fact_program_hash);
        keccak_input.append(state_transition_fact);
        keccak_u256s_solidity_inputs(keccak_input.span())
    }

    fn hash_main_public_input_solidity(program_output: Span<felt252>) -> u256 {
        let mut keccak_input: Array<u256> = ArrayTrait::new();
        let mut i = 0;
        loop {
            if (i == program_output.len()) {
                break;
            }
            keccak_input.append((*program_output.at(i)).into());
            i += 1;
        }

        keccak_u256s_solidity_inputs(keccak_input.span())
    }

    fn keccak_u256s_solidity_inputs(input: Span<u256>) -> u256 {
        byte_reverse_u256(core::keccak::keccak_u256s_be_inputs(input))
    }

    fn byte_reverse_u256(value: u256) -> u256 {
        u256 {
            low: core::integer::u128_byte_reverse(value.high),
            high: core::integer::u128_byte_reverse(value.low),
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
