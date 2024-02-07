//! SPDX-License-Identifier: MIT
//!
//! Interface for Appchain - Starknet messaging.
use starknet::ContractAddress;

#[starknet::interface]
trait IMessaging<T> {
    /// Sends a message to the Appchain from Starknet.
    ///
    /// <https://github.com/starkware-libs/cairo-lang/blob/caba294d82eeeccc3d86a158adb8ba209bf2d8fc/src/starkware/starknet/solidity/StarknetMessaging.sol#L110>.
    ///
    /// # Arguments
    ///
    /// * `to_address` - Contract address to send the message to on the Appchain.
    /// * `selector` - The `l1_handler` function selector of the contract on the Appchain to execute.
    /// * `payload` - The message payload.
    ///
    /// # Returns
    ///
    /// The message hash and the updated nonce of the message.
    fn send_message_to_appchain(
        ref self: T, to_address: ContractAddress, selector: felt252, payload: Span<felt252>
    ) -> (felt252, felt252);

    /// Consumes a message received from a state update of the Appchain.
    ///
    /// <https://github.com/starkware-libs/cairo-lang/blob/caba294d82eeeccc3d86a158adb8ba209bf2d8fc/src/starkware/starknet/solidity/StarknetMessaging.sol#L132>.
    ///
    /// # Arguments
    ///
    /// * `from_address` - Contract address from which the message was sent on the Appchain.
    /// * `payload` - The message payload.
    ///
    /// # Returns
    ///
    /// Returns the hash of the consummed message.
    fn consume_message_from_appchain(
        ref self: T, from_address: ContractAddress, payload: Span<felt252>
    ) -> felt252;

    /// Starts the cancellation procedure for a message sent from
    /// Starknet to the Appchain.
    ///
    /// Once the cancellation delay is over, the message can be cancelled.
    ///
    /// <https://github.com/starkware-libs/cairo-lang/blob/caba294d82eeeccc3d86a158adb8ba209bf2d8fc/src/starkware/starknet/solidity/StarknetMessaging.sol#L147>.
    ///
    /// # Arguments
    ///
    /// * `to_address` - Contract address to send the message to on the Appchain.
    /// * `selector` - The `l1_handler` function selector of the contract on the Appchain to execute.
    /// * `payload` - The message's payload.
    /// * `nonce` - The message's nonce.
    ///
    /// # Returns
    ///
    /// The hash of the message.
    fn start_message_cancellation(
        ref self: T,
        to_address: ContractAddress,
        selector: felt252,
        payload: Span<felt252>,
        nonce: felt252,
    ) -> felt252;

    /// Cancels a message from Starknet to Appchain if the cancellation delays has expired.
    ///
    /// <https://github.com/starkware-libs/cairo-lang/blob/caba294d82eeeccc3d86a158adb8ba209bf2d8fc/src/starkware/starknet/solidity/StarknetMessaging.sol#L161>.
    ///
    /// # Arguments
    ///
    /// * `to_address` - Contract address to send the message to on the Appchain.
    /// * `selector` - The `l1_handler` function selector of the contract on the Appchain to execute.
    /// * `payload` - The message's payload.
    /// * `nonce` - The message's nonce.
    ///
    /// # Returns
    ///
    /// The hash of the message.
    fn cancel_message(
        ref self: T,
        to_address: ContractAddress,
        selector: felt252,
        payload: Span<felt252>,
        nonce: felt252,
    ) -> felt252;
}
