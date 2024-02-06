//! SPDX-License-Identifier: MIT
//!
//! Appchain - Starknet messaging component.
//!
//! Messaging between the appchain and Starknet works very similar
//! to Starknet - Ethereum messaging.
//!
//! # Starknet to Appchain message
//!
//! To send a message from Starknet to the Appchain, the current contract
//! emits an event. This event is then retrived by the Appchain sequencer
//! to execute the `l1_handler` function of the target contract.
//!
//! # Appchain to Starknet message
//!
//! To send a message from the Appchain to Starknet, the Appchain sequencer
//! gathers all the messages created by cairo contracts using the `send_message_to_l1`
//! syscall. Those messages are then sent to Starknet as part of the state update once
//! the proof is available.
//! Once taken in account during the state update, the message can be be consummed
//! on Starknet.
//!
//! As the messages are part of the state update which depends on the proof,
//! the messaging system between Appchain and Starknet is also asynchronous and
//! asymmetric as the Starknet - Ethereum messaging.

/// Errors.
mod errors {
    const INVALID_MESSAGE_TO_CONSUME: felt252 = 'INVALID_MESSAGE_TO_CONSUME';
}

/// Messaging component.
///
/// Depends on `ownable` to ensure the configuration is
/// only editable by contract's owner.
#[starknet::component]
mod messaging_cpt {
    use core::zeroable::Zeroable;
    use starknet::ContractAddress;

    use openzeppelin::access::ownable::{
        OwnableComponent as ownable_cpt, OwnableComponent::InternalTrait as OwnableInternal,
        interface::IOwnable,
    };

    use piltover::messaging::interface::IMessaging;

    use super::errors;

    #[storage]
    struct Storage {
        /// The nonce for messages sent to the Appchain from Starknet.
        sn_to_appc_nonce: felt252,
        /// Ledger of messages hashes sent from Starknet to the appchain.
        sn_to_appc_messages: LegacyMap::<felt252, felt252>,
        /// Ledger of messages hashes registered from the appchain and a refcount
        /// associated to it to control messages consumption.
        appc_to_sn_messages: LegacyMap::<felt252, felt252>,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        MessageSent: MessageSent,
        MessageConsumed: MessageConsumed,
    }

    #[derive(Drop, starknet::Event)]
    struct MessageSent {
        #[key]
        message_hash: felt252,
        #[key]
        from: ContractAddress,
        #[key]
        to: ContractAddress,
        selector: felt252,
        nonce: felt252,
        payload: Span<felt252>,
    }

    #[derive(Drop, starknet::Event)]
    struct MessageConsumed {
        #[key]
        message_hash: felt252,
        #[key]
        from: ContractAddress,
        #[key]
        to: ContractAddress,
        payload: Span<felt252>,
    }

    #[embeddable_as(MessagingImpl)]
    impl Messaging<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of IMessaging<ComponentState<TContractState>> {
        fn send_message_to_appchain(
            ref self: ComponentState<TContractState>,
            to_address: ContractAddress,
            selector: felt252,
            payload: Span<felt252>
        ) -> (felt252, felt252) {
            let nonce = self.sn_to_appc_nonce.read();
            self.sn_to_appc_nonce.write(nonce + 1);

            let message_hash = self
                .compute_message_hash_sn_to_appc(nonce, to_address, selector, payload);

            self
                .emit(
                    MessageSent {
                        message_hash,
                        from: starknet::get_caller_address(),
                        to: to_address,
                        selector,
                        nonce,
                        payload,
                    }
                );

            // L1 starknet core contract stores the msg.value + 1.
            // TODO: is the nonce a good thing to keep here then?
            self.sn_to_appc_messages.write(message_hash, nonce);
            (message_hash, nonce)
        }

        fn consume_message_from_appchain(
            ref self: ComponentState<TContractState>,
            from_address: ContractAddress,
            payload: Span<felt252>
        ) -> felt252 {
            let to_address = starknet::get_caller_address();

            let message_hash = self
                .compute_message_hash_appc_to_sn(from_address, to_address, payload);

            let count = self.appc_to_sn_messages.read(message_hash);
            assert(count.is_non_zero(), errors::INVALID_MESSAGE_TO_CONSUME);

            self
                .emit(
                    MessageConsumed { message_hash, from: from_address, to: to_address, payload, }
                );

            self.appc_to_sn_messages.write(message_hash, count - 1);

            message_hash
        }
    }

    #[generate_trait]
    impl InternalImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of InternalTrait<TContractState> {
        /// Computes the hash of a message that is sent from Starknet to the Appchain.
        ///
        /// <https://github.com/starkware-libs/cairo-lang/blob/caba294d82eeeccc3d86a158adb8ba209bf2d8fc/src/starkware/starknet/solidity/StarknetMessaging.sol#L88>
        ///
        /// # Arguments
        ///
        /// * `nonce` - Nonce of the message.
        /// * `to_address` - Contract address to send the message to on the Appchain.
        /// * `selector` - The `l1_handler` function selector of the contract on the Appchain
        ///                to execute.
        /// * `payload` - The message payload.
        ///
        /// # Returns
        ///
        /// The hash of the message from Starknet to the Appchain.
        fn compute_message_hash_sn_to_appc(
            ref self: ComponentState<TContractState>,
            nonce: felt252,
            to_address: ContractAddress,
            selector: felt252,
            payload: Span<felt252>
        ) -> felt252 {
            let mut hash_data = array![nonce, to_address.into(), selector,];

            let mut i = 0_usize;
            loop {
                if i == payload.len() {
                    break;
                }
                hash_data.append((*payload[i]));
                i += 1;
            };

            core::poseidon::poseidon_hash_span(hash_data.span())
        }

        /// Computes the hash of a message that is sent from the Appchain to Starknet.
        ///
        /// <https://github.com/starkware-libs/cairo-lang/blob/caba294d82eeeccc3d86a158adb8ba209bf2d8fc/src/starkware/starknet/solidity/StarknetMessaging.sol#L137>
        ///
        /// # Arguments
        ///
        /// * `from_address` - Contract address of the message sender on the Appchain.
        /// * `to_address` - Contract address to send the message to on the Appchain.
        /// * `payload` - The message payload.
        ///
        /// # Returns
        ///
        /// The hash of the message from the Appchain to Starknet.
        fn compute_message_hash_appc_to_sn(
            ref self: ComponentState<TContractState>,
            from_address: ContractAddress,
            to_address: ContractAddress,
            payload: Span<felt252>
        ) -> felt252 {
            let mut hash_data: Array<felt252> = array![
                from_address.into(), to_address.into(), payload.len().into(),
            ];

            let mut i = 0_usize;
            loop {
                if i == payload.len() {
                    break;
                }
                hash_data.append((*payload[i]));
                i += 1;
            };

            core::poseidon::poseidon_hash_span(hash_data.span())
        }
    }
}
