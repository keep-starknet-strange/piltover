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
    pub const INVALID_MESSAGE_TO_CONSUME: felt252 = 'INVALID_MESSAGE_TO_CONSUME';
    pub const INVALID_MESSAGE_TO_SEAL: felt252 = 'INVALID_MESSAGE_TO_SEAL';
    pub const NO_MESSAGE_TO_CANCEL: felt252 = 'NO_MESSAGE_TO_CANCEL';
    pub const CANCELLATION_INVALID_TIMESTAMP: felt252 = 'CANCELLATION_INVALID_TIMESTAMP';
    pub const CANCELLATION_NOT_ALLOWED_YET: felt252 = 'CANCELLATION_NOT_ALLOWED_YET';
    pub const CANCEL_ALLOWED_TIME_OVERFLOW: felt252 = 'CANCEL_ALLOWED_TIME_OVERFLOW';
    pub const CANCELLATION_ALREADY_REQUESTED: felt252 = 'CANCELLATION_ALREADY_REQUESTED';
    pub const CANCELLATION_ALREADY_DONE: felt252 = 'CANCELLATION_ALREADY_DONE';
}

/// Messaging component.
///
/// Depends on `ownable` to ensure the configuration is
/// only editable by contract's owner.
#[starknet::component]
pub mod messaging_cpt {
    use core::num::traits::Zero;
    #[cfg(feature: 'messaging_test')]
    use piltover::messaging::IMessagingTest;
    use piltover::messaging::{
        hash, interface::IMessaging,
        types::{MessageHash, MessageToAppchainStatus, MessageToStarknetStatus, Nonce},
    };
    use piltover::snos_output::{MessageToAppchain, MessageToStarknet};
    use starknet::ContractAddress;
    use starknet::storage::Map;
    use starknet::storage::{
        StorageMapReadAccess, StorageMapWriteAccess, StoragePointerReadAccess,
        StoragePointerWriteAccess,
    };
    use super::errors;

    #[storage]
    pub struct Storage {
        /// Cancellation delay in seconds for message from Starknet to Appchain.
        pub cancellation_delay_secs: u64,
        /// Ledger of messages from Starknet to Appchain that are being cancelled.
        pub sn_to_appc_cancellations: Map::<MessageHash, u64>,
        /// The nonce for messages sent to the Appchain from Starknet.
        pub sn_to_appc_nonce: Nonce,
        /// Ledger of messages hashes sent from Starknet to the appchain and their status.
        pub sn_to_appc_messages: Map::<MessageHash, MessageToAppchainStatus>,
        /// Ledger of messages hashes registered from the Appchain and a refcount
        /// associated to it to control messages consumption.
        pub appc_to_sn_messages: Map::<MessageHash, felt252>,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {
        MessageSent: MessageSent,
        MessageConsumed: MessageConsumed,
        MessageCancellationStarted: MessageCancellationStarted,
        MessageCanceled: MessageCanceled,
        MessageToStarknetReceived: MessageToStarknetReceived,
        MessageToAppchainSealed: MessageToAppchainSealed,
    }

    #[derive(Drop, starknet::Event)]
    pub struct MessageSent {
        #[key]
        pub message_hash: MessageHash,
        #[key]
        pub from: ContractAddress,
        #[key]
        pub to: ContractAddress,
        pub selector: felt252,
        pub nonce: Nonce,
        pub payload: Span<felt252>,
    }

    #[derive(Drop, starknet::Event)]
    pub struct MessageConsumed {
        #[key]
        pub message_hash: MessageHash,
        #[key]
        pub from: ContractAddress,
        #[key]
        pub to: ContractAddress,
        pub payload: Span<felt252>,
    }

    #[derive(Drop, starknet::Event)]
    pub struct MessageCancellationStarted {
        #[key]
        pub message_hash: MessageHash,
        #[key]
        pub from: ContractAddress,
        #[key]
        pub to: ContractAddress,
        pub selector: felt252,
        pub payload: Span<felt252>,
        pub nonce: Nonce,
    }

    #[derive(Drop, starknet::Event)]
    pub struct MessageCanceled {
        #[key]
        pub message_hash: MessageHash,
        #[key]
        pub from: ContractAddress,
        #[key]
        pub to: ContractAddress,
        pub selector: felt252,
        pub payload: Span<felt252>,
        pub nonce: Nonce,
    }

    #[derive(Drop, starknet::Event)]
    pub struct MessageToStarknetReceived {
        #[key]
        pub message_hash: MessageHash,
        #[key]
        pub from: ContractAddress,
        #[key]
        pub to: ContractAddress,
        pub payload: Span<felt252>,
    }

    #[derive(Drop, starknet::Event)]
    pub struct MessageToAppchainSealed {
        #[key]
        pub message_hash: MessageHash,
        #[key]
        pub from: ContractAddress,
        #[key]
        pub to: ContractAddress,
        pub selector: felt252,
        pub nonce: Nonce,
        pub payload: Span<felt252>,
    }

    #[embeddable_as(MessagingImpl)]
    impl Messaging<
        TContractState, +HasComponent<TContractState>,
    > of IMessaging<ComponentState<TContractState>> {
        fn send_message_to_appchain(
            ref self: ComponentState<TContractState>,
            to_address: ContractAddress,
            selector: felt252,
            payload: Span<felt252>,
        ) -> (MessageHash, Nonce) {
            // From the what's done from L1 to L2, the first nonce must be 0:
            // <https://github.com/starkware-libs/cairo-lang/blob/caba294d82eeeccc3d86a158adb8ba209bf2d8fc/src/starkware/starknet/solidity/StarknetMessaging.sol#L117>
            // The value is read from the storage, and then incremented but the read
            // value is used at the tx nonce.
            let mut nonce = self.sn_to_appc_nonce.read();

            // Increment the nonce, but the read value is used at the tx nonce.
            self.sn_to_appc_nonce.write(nonce + 1);

            let from = starknet::get_caller_address();
            let message_hash = hash::compute_message_hash_sn_to_appc(
                from, to_address, selector, payload, nonce,
            );

            self.emit(MessageSent { message_hash, from, to: to_address, selector, nonce, payload });

            self.sn_to_appc_messages.write(message_hash, MessageToAppchainStatus::Pending(nonce));
            (message_hash, nonce)
        }

        fn sn_to_appchain_messages(
            self: @ComponentState<TContractState>, message_hash: MessageHash,
        ) -> MessageToAppchainStatus {
            self.sn_to_appc_messages.read(message_hash)
        }

        fn appchain_to_sn_messages(
            self: @ComponentState<TContractState>, message_hash: MessageHash,
        ) -> MessageToStarknetStatus {
            let message_count = self.appc_to_sn_messages.read(message_hash);
            if (message_count == 0) {
                return MessageToStarknetStatus::NothingToConsume;
            }
            return MessageToStarknetStatus::ReadyToConsume(message_count);
        }

        fn consume_message_from_appchain(
            ref self: ComponentState<TContractState>,
            from_address: ContractAddress,
            payload: Span<felt252>,
        ) -> MessageHash {
            let to_address = starknet::get_caller_address();

            let message_hash = hash::compute_message_hash_appc_to_sn(
                from_address, to_address, payload,
            );

            let count = self.appc_to_sn_messages.read(message_hash);
            assert(count.is_non_zero(), errors::INVALID_MESSAGE_TO_CONSUME);

            self
                .emit(
                    MessageConsumed { message_hash, from: from_address, to: to_address, payload },
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
            let from = starknet::get_caller_address();

            let message_hash = hash::compute_message_hash_sn_to_appc(
                from, to_address, selector, payload, nonce,
            );

            match self.sn_to_appc_messages.read(message_hash) {
                MessageToAppchainStatus::Pending(_) => {},
                MessageToAppchainStatus::Cancelled => {
                    core::panic_with_felt252(errors::CANCELLATION_ALREADY_DONE)
                },
                MessageToAppchainStatus::Cancelling => {
                    core::panic_with_felt252(errors::CANCELLATION_ALREADY_REQUESTED)
                },
                _ => assert(false, errors::NO_MESSAGE_TO_CANCEL),
            };

            self.sn_to_appc_cancellations.write(message_hash, starknet::get_block_timestamp());
            self.sn_to_appc_messages.write(message_hash, MessageToAppchainStatus::Cancelling);

            self
                .emit(
                    MessageCancellationStarted {
                        message_hash, from, to: to_address, selector, payload, nonce,
                    },
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

            let message_hash = hash::compute_message_hash_sn_to_appc(
                from, to_address, selector, payload, nonce,
            );

            assert(
                self.sn_to_appc_messages.read(message_hash) == MessageToAppchainStatus::Cancelling,
                errors::NO_MESSAGE_TO_CANCEL,
            );

            let request_time = self.sn_to_appc_cancellations.read(message_hash);
            assert(request_time.is_non_zero(), errors::CANCELLATION_INVALID_TIMESTAMP);

            let cancel_allowed_time = request_time + self.cancellation_delay_secs.read();
            assert(cancel_allowed_time >= request_time, errors::CANCEL_ALLOWED_TIME_OVERFLOW);
            assert(
                starknet::get_block_timestamp() >= cancel_allowed_time,
                errors::CANCELLATION_NOT_ALLOWED_YET,
            );

            self
                .emit(
                    MessageCanceled {
                        message_hash, from, to: to_address, selector, payload, nonce,
                    },
                );

            // Once canceled, no more operation can be done on this message.
            self.sn_to_appc_messages.write(message_hash, MessageToAppchainStatus::Cancelled);

            return message_hash;
        }
    }

    #[cfg(feature: 'messaging_test')]
    #[embeddable_as(MessagingTestImpl)]
    impl MessagingTest<
        TContractState, +HasComponent<TContractState>,
    > of IMessagingTest<ComponentState<TContractState>> {
        fn add_messages_hashes_from_appchain(
            ref self: ComponentState<TContractState>, messages_hashes: Span<felt252>,
        ) {
            let mut i = 0_usize;
            loop {
                if i == messages_hashes.len() {
                    break;
                }

                let msg_hash = *messages_hashes[i];

                let count = self.appc_to_sn_messages.read(msg_hash);
                self.appc_to_sn_messages.write(msg_hash, count + 1);

                // We can't have the detail of the message here, so we emit a dummy event
                // with at least the message hash.
                self
                    .emit(
                        MessageToStarknetReceived {
                            message_hash: msg_hash,
                            from: 0.try_into().unwrap(),
                            to: 0.try_into().unwrap(),
                            payload: array![].span(),
                        },
                    );

                i += 1;
            };
        }
    }

    #[generate_trait]
    pub impl InternalImpl<
        TContractState, +HasComponent<TContractState>,
    > of InternalTrait<TContractState> {
        /// Initializes the messaging component.
        ///
        /// # Arguments
        ///
        /// * `cancellation_delay_secs` - The delay in seconds for message cancellation window.
        fn initialize(ref self: ComponentState<TContractState>, cancellation_delay_secs: u64) {
            self.cancellation_delay_secs.write(cancellation_delay_secs);
        }

        /// Processes the messages to Starknet from StarknetOS output.
        /// Once processed, messages are ready to be consumed using
        /// `consume_message_from_appchain` entry point.
        ///
        /// # Arguments
        ///
        /// * `messages` - The messages to Starknet.
        fn process_messages_to_starknet(
            ref self: ComponentState<TContractState>, messages: Span<MessageToStarknet>,
        ) {
            let mut messages = messages;

            loop {
                match messages.pop_front() {
                    Option::Some(m) => {
                        let from = *m.from_address;
                        let to = *m.to_address;
                        let payload = *m.payload;

                        let message_hash = hash::compute_message_hash_appc_to_sn(from, to, payload);

                        self.emit(MessageToStarknetReceived { message_hash, from, to, payload });

                        let ref_count = self.appc_to_sn_messages.read(message_hash);
                        self.appc_to_sn_messages.write(message_hash, ref_count + 1);
                    },
                    Option::None => { break; },
                };
            };
        }

        /// Processes the messages to Appchain from StarknetOS output.
        ///
        /// # Arguments
        ///
        /// * `messages` - The messages to Appchain.
        fn process_messages_to_appchain(
            ref self: ComponentState<TContractState>, messages: Span<MessageToAppchain>,
        ) {
            let mut messages = messages;

            loop {
                match messages.pop_front() {
                    Option::Some(m) => {
                        let from = *m.from_address;
                        let to = *m.to_address;
                        let payload = *m.payload;
                        let selector = *m.selector;
                        let nonce = *m.nonce;

                        let message_hash = hash::compute_message_hash_sn_to_appc(
                            from, to, selector, payload, nonce,
                        );

                        match self.sn_to_appc_messages.read(message_hash) {
                            MessageToAppchainStatus::Pending(_) => {},
                            _ => assert(false, errors::INVALID_MESSAGE_TO_SEAL),
                        };

                        // On the L1, they use the Fee in front of the message hash, not the nonce.
                        // Here, we have an enum to explicitly indicate that the message is sealed.
                        self
                            .sn_to_appc_messages
                            .write(message_hash, MessageToAppchainStatus::Sealed);

                        self
                            .emit(
                                MessageToAppchainSealed {
                                    message_hash, from, to, selector, payload, nonce,
                                },
                            );
                    },
                    Option::None => { break; },
                };
            };
        }
    }
}
