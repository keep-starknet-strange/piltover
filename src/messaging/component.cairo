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
    const INVALID_NONCE: felt252 = 'INVALID_NONCE';
    const INVALID_MESSAGE_TO_CONSUME: felt252 = 'INVALID_MESSAGE_TO_CONSUME';
    const NO_MESSAGE_TO_CANCEL: felt252 = 'NO_MESSAGE_TO_CANCEL';
    const CANCELLATION_NOT_REQUESTED: felt252 = 'CANCELLATION_NOT_REQUESTED';
    const CANCELLATION_NOT_ALLOWED_YET: felt252 = 'CANCELLATION_NOT_ALLOWED_YET';
    const CANCEL_ALLOWED_TIME_OVERFLOW: felt252 = 'CANCEL_ALLOWED_TIME_OVERFLOW';
}

/// Messaging component.
///
/// Depends on `ownable` to ensure the configuration is
/// only editable by contract's owner.
#[starknet::component]
mod messaging_cpt {
    use core::zeroable::Zeroable;
    use openzeppelin::access::ownable::{
        OwnableComponent as ownable_cpt, OwnableComponent::InternalTrait as OwnableInternal,
        interface::IOwnable,
    };
    use piltover::messaging::interface::IMessaging;
    use starknet::ContractAddress;
    use super::errors;

    type MessageHash = felt252;
    type Nonce = felt252;

    #[storage]
    struct Storage {
        /// Cancellation delay in seconds for message from Starknet to Appchain.
        cancellation_delay_secs: u64,
        /// Ledger of messages from Starknet to Appchain that are being cancelled.
        sn_to_appc_cancellations: LegacyMap::<MessageHash, u64>,
        /// The nonce for messages sent to the Appchain from Starknet.
        sn_to_appc_nonce: Nonce,
        /// Ledger of messages hashes sent from Starknet to the appchain.
        sn_to_appc_messages: LegacyMap::<MessageHash, Nonce>,
        /// Ledger of messages hashes registered from the Appchain and a refcount
        /// associated to it to control messages consumption.
        appc_to_sn_messages: LegacyMap::<MessageHash, felt252>,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        MessageSent: MessageSent,
        MessageConsumed: MessageConsumed,
        MessageCancellationStarted: MessageCancellationStarted,
        MessageCanceled: MessageCanceled,
    }

    #[derive(Drop, starknet::Event)]
    struct MessageSent {
        #[key]
        message_hash: MessageHash,
        #[key]
        from: ContractAddress,
        #[key]
        to: ContractAddress,
        selector: felt252,
        nonce: Nonce,
        payload: Span<felt252>,
    }

    #[derive(Drop, starknet::Event)]
    struct MessageConsumed {
        #[key]
        message_hash: MessageHash,
        #[key]
        from: ContractAddress,
        #[key]
        to: ContractAddress,
        payload: Span<felt252>,
    }

    #[derive(Drop, starknet::Event)]
    struct MessageCancellationStarted {
        #[key]
        message_hash: MessageHash,
        #[key]
        from: ContractAddress,
        #[key]
        to: ContractAddress,
        selector: felt252,
        payload: Span<felt252>,
        nonce: Nonce,
    }

    #[derive(Drop, starknet::Event)]
    struct MessageCanceled {
        #[key]
        message_hash: MessageHash,
        #[key]
        from: ContractAddress,
        #[key]
        to: ContractAddress,
        selector: felt252,
        payload: Span<felt252>,
        nonce: Nonce,
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
        ) -> (MessageHash, Nonce) {
            // Starts by +1 to avoid 0 as a valid nonce.
            let nonce = self.sn_to_appc_nonce.read() + 1;
            self.sn_to_appc_nonce.write(nonce);

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

            self.sn_to_appc_messages.write(message_hash, nonce);
            (message_hash, nonce)
        }

        fn consume_message_from_appchain(
            ref self: ComponentState<TContractState>,
            from_address: ContractAddress,
            payload: Span<felt252>
        ) -> MessageHash {
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

        fn start_message_cancellation(
            ref self: ComponentState<TContractState>,
            to_address: ContractAddress,
            selector: felt252,
            payload: Span<felt252>,
            nonce: Nonce,
        ) -> MessageHash {
            assert(nonce.is_non_zero(), errors::INVALID_NONCE);
            let from = starknet::get_caller_address();

            let message_hash = self
                .compute_message_hash_sn_to_appc(nonce, to_address, selector, payload);

            assert(
                self.sn_to_appc_messages.read(message_hash) == nonce, errors::NO_MESSAGE_TO_CANCEL
            );

            self.sn_to_appc_cancellations.write(message_hash, starknet::get_block_timestamp());

            self
                .emit(
                    MessageCancellationStarted {
                        message_hash, from, to: to_address, selector, payload, nonce
                    }
                );

            return message_hash;
        }

        fn cancel_message(
            ref self: ComponentState<TContractState>,
            to_address: ContractAddress,
            selector: felt252,
            payload: Span<felt252>,
            nonce: felt252,
        ) -> MessageHash {
            let from = starknet::get_caller_address();

            let message_hash = self
                .compute_message_hash_sn_to_appc(nonce, to_address, selector, payload);

            assert(
                self.sn_to_appc_messages.read(message_hash) == nonce, errors::NO_MESSAGE_TO_CANCEL
            );

            let request_time = self.sn_to_appc_cancellations.read(message_hash);
            assert(request_time.is_non_zero(), errors::CANCELLATION_NOT_REQUESTED);

            let cancel_allowed_time = request_time + self.cancellation_delay_secs.read();
            assert(cancel_allowed_time >= request_time, errors::CANCEL_ALLOWED_TIME_OVERFLOW);
            assert(
                starknet::get_block_timestamp() >= cancel_allowed_time,
                errors::CANCELLATION_NOT_ALLOWED_YET
            );

            self
                .emit(
                    MessageCanceled { message_hash, from, to: to_address, selector, payload, nonce }
                );

            // Once canceled, no more operation can be done on this message.
            self.sn_to_appc_messages.write(message_hash, 0);

            return message_hash;
        }
    }

    #[generate_trait]
    impl InternalImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of InternalTrait<TContractState> {
        /// Initializes the messaging component.
        ///
        /// # Arguments
        ///
        /// * `cancellation_delay_secs` - The delay in seconds for message cancellation window.
        fn initialize(ref self: ComponentState<TContractState>, cancellation_delay_secs: u64) {
            self.cancellation_delay_secs.write(cancellation_delay_secs);
        }

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
            nonce: Nonce,
            to_address: ContractAddress,
            selector: felt252,
            payload: Span<felt252>
        ) -> MessageHash {
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
        ) -> MessageHash {
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
