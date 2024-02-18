//! SPDX-License-Identifier: MIT
//!
//!

mod errors {
    const INVALID_ADDRESS: felt252 = 'Config: invalid address';
    const SNOS_INVALID_PROGRAM_OUTPUT_SIZE: felt252 = 'snos: invalid output size';
    const SNOS_INVALID_MESSAGES_SEGMENTS: felt252 = 'snos: invalid messages segments';
}

/// Appchain settlement contract on starknet.
#[starknet::contract]
mod appchain {
    use openzeppelin::access::ownable::{
        OwnableComponent as ownable_cpt, OwnableComponent::InternalTrait as OwnableInternal,
        interface::IOwnable
    };
    use piltover::config::{config_cpt, config_cpt::InternalTrait as ConfigInternal, IConfig};
    use piltover::interface::IAppchain;
    use piltover::messaging::{
        messaging_cpt, messaging_cpt::InternalTrait as MessagingInternal, IMessaging,
        output_process, output_process::{MessageToStarknet, MessageToAppchain},
    };
    use piltover::onchain_data_fact_tree_encoder::onchain_data_fact_tree_encoder::OnchainDataFactTreeEncoder::{
        encode_fact_with_onchain_data, data_availability_fact
    };
    use piltover::snos_output;
    use starknet::ContractAddress;
    use super::errors;

    /// The default cancellation delay of 5 days.
    const CANCELLATION_DELAY_SECS: u64 = 432000;

    component!(path: ownable_cpt, storage: ownable, event: OwnableEvent);
    component!(path: config_cpt, storage: config, event: ConfigEvent);
    component!(path: messaging_cpt, storage: messaging, event: MessagingEvent);

    #[abi(embed_v0)]
    impl ConfigImpl = config_cpt::ConfigImpl<ContractState>;
    #[abi(embed_v0)]
    impl MessagingImpl = messaging_cpt::MessagingImpl<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        ownable: ownable_cpt::Storage,
        #[substorage(v0)]
        config: config_cpt::Storage,
        #[substorage(v0)]
        messaging: messaging_cpt::Storage,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        OwnableEvent: ownable_cpt::Event,
        #[flat]
        ConfigEvent: config_cpt::Event,
        #[flat]
        MessagingEvent: messaging_cpt::Event,
    }

    /// Initializes the contract.
    ///
    /// # Arguments
    ///
    /// * `address` - The contract address of the owner.
    #[constructor]
    fn constructor(ref self: ContractState, owner: ContractAddress) {
        self.ownable.initializer(owner);
        self.messaging.initialize(CANCELLATION_DELAY_SECS);
    }

    // Updates the state of the Starknet, based on a proof of the Starknet OS that the state transition is valid.
    // Data availability is provided on-chain.
    // programOutput - The main part of the StarkNet OS program output.
    // data_availability_fact - An encoding of the on-chain data associated with the 'programOutput'.

    #[abi(embed_v0)]
    impl Appchain of IAppchain<ContractState> {
        fn update_state(
            ref self: ContractState,
            program_output: Span<felt252>,
            onchain_data_hash: felt252,
            onchain_data_size: u256
        ) {
            self.config.assert_only_owner_or_operator();

            // TODO(#2): reentrancy guard.
            // TODO(#3): facts verification.
            // TODO(#4): update the current state (component needed).

            // Header size + 2 messages segments len.
            assert(
                program_output.len() > snos_output::HEADER_SIZE + 2,
                errors::SNOS_INVALID_PROGRAM_OUTPUT_SIZE
            );
            let value: data_availability_fact = data_availability_fact {
                onchain_data_hash: onchain_data_hash, onchain_data_size: onchain_data_size
            };
            let state_transition_fact: u256 = encode_fact_with_onchain_data(program_output, value);
            let mut offset = snos_output::HEADER_SIZE;

            // TODO(#7): We should update SNOS output to have the messages count
            // instead of the messages segment len.

            let mut messages_segments = program_output.slice(offset, program_output.len() - offset);

            let (messages_to_starknet, messages_to_appchain) =
                output_process::gather_messages_from_output(
                messages_segments
            );

            self.messaging.process_messages_to_starknet(messages_to_starknet);
            self.messaging.process_messages_to_appchain(messages_to_appchain);
        }
    }
}
