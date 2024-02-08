//! SPDX-License-Identifier: MIT
//!
//! Helpers functions to process messages from
//! state update buffer.
//!
//! The output of StarknetOS can be found here:
//! <https://github.com/starkware-libs/cairo-lang/blob/caba294d82eeeccc3d86a158adb8ba209bf2d8fc/src/starkware/starknet/core/os/output.cairo#L41>
//!
//! Solidity code related to message processing:
//! <https://github.com/starkware-libs/cairo-lang/blob/caba294d82eeeccc3d86a158adb8ba209bf2d8fc/src/starkware/starknet/solidity/Output.sol>.
//!
use starknet::ContractAddress;

/// Message to Starknet.
#[derive(Drop, Serde)]
struct MessageToStarknet {
    /// Appchain contract address sending the message.
    from_address: ContractAddress,
    /// Starknet contract address receiving the message.
    to_address: ContractAddress,
    /// Payload of the message.
    payload: Span<felt252>,
}

/// Message to Appchain.
#[derive(Drop, Serde)]
struct MessageToAppchain {
    /// Starknet address sending the message.
    from_address: ContractAddress,
    /// Appchain address receiving the message.
    to_address: ContractAddress,
    /// Nonce.
    nonce: felt252,
    /// Function selector (with #[l1 handler] attribute).
    selector: felt252,
    /// Payload size.
    payload: Span<felt252>,
}

const MESSAGE_TO_STARKNET_HEADER_SIZE: usize = 3;
const MESSAGE_TO_APPCHAIN_HEADER_SIZE: usize = 5;

/// Function to gather the messages from SNOS output.
/// TODO: this must be removed if SNOS output is changed to use messages
/// count instead of segment length.
///
/// # Argument
///
/// * `output_messages` - The messages segments of the SNOS output.
///
/// # Returns
///
/// A tuple with the messages to Starknet and messages to Appchain
/// deserialized.
fn gather_messages_from_output(output_messages: Span<felt252>) -> (Span<MessageToStarknet>, Span<MessageToAppchain>) {
    let mut messages_to_starknet = array![];
    let mut messages_to_appchain = array![];

    // Messages to Starknet.
    let segment_len: usize = (*output_messages[0]).try_into().expect('invalid seg size sn');

    let mut offset = 1;
    let segment_end = offset + segment_len;

    if segment_end > output_messages.len() {
        core::panic_with_felt252('invalid message segment sn');
    }

    loop {
        if offset >= segment_end {
            break; 
        }

        if offset + MESSAGE_TO_STARKNET_HEADER_SIZE > segment_end {
            core::panic_with_felt252('invalid message sn');
        }

        // +2 due to fields arrangement.
        let payload_size: usize = (*output_messages[offset + 2]).try_into().expect('invalid size sn');

        // +1 for payload size and +2 for remaining fields.
        let mut slice = output_messages.slice(offset, payload_size + 1 + 2);

        let m: MessageToStarknet = Serde::deserialize(ref slice).expect('bad format message sn');

        messages_to_starknet.append(m);

        offset += payload_size + 1 + 2;
    };

    // Messages to Appchain.
    let segment_len: usize = (*output_messages[offset]).try_into().expect('invalid seg size appc');

    // Move to the first message.
    offset += 1;
    let segment_end = offset + segment_len;

    if segment_end > output_messages.len() {
        core::panic_with_felt252('invalid message segment appc');
    }

    loop {
        if offset >= segment_end {
            break; 
        }

        if offset + MESSAGE_TO_APPCHAIN_HEADER_SIZE > segment_end {
            core::panic_with_felt252('invalid message appc');
        }

        // +4 due to fields arrangement.
        let payload_size: usize = (*output_messages[offset + 4]).try_into().expect('invalid size appc');

        // +1 for payload size and +4 for remaining fields.
        let mut slice = output_messages.slice(offset, payload_size + 1 + 4);

        let m: MessageToAppchain = Serde::deserialize(ref slice).expect('bad format message appc');

        messages_to_appchain.append(m);

        offset += payload_size + 1 + 4;
    };

    (messages_to_starknet.span(), messages_to_appchain.span())
}
