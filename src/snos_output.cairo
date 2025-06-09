//! SNOS output related types and variables.
//!
use core::array::SpanIter;
use core::iter::IntoIterator;
use core::iter::Iterator;
use core::num::traits::Zero;
use starknet::ContractAddress;

/// Size of the header of the output of SNOS.
const HEADER_SIZE: usize = 10;
/// Size of the header of a message to Starknet, which is
/// right before the payload content.
const MESSAGE_TO_STARKNET_HEADER_SIZE: usize = 3;
/// Size of the header of a message to appchain, which is
/// right before the payload content.
const MESSAGE_TO_APPCHAIN_HEADER_SIZE: usize = 5;

const PREVIOUS_MERKLE_UPDATE_OFFSET: usize = 0;
const NEW_MERKLE_UPDATE_OFFSET: usize = 1;
const PREV_BLOCK_NUMBER_OFFSET: usize = 2;
const NEW_BLOCK_NUMBER_OFFSET: usize = 3;
const PREV_BLOCK_HASH_OFFSET: usize = 4;
const NEW_BLOCK_HASH_OFFSET: usize = 5;
const OS_PROGRAM_HASH_OFFSET: usize = 6;
const CONFIG_HASH_OFFSET: usize = 7;
const USE_KZG_DA_OFFSET: usize = 8;
const FULL_OUTPUT_OFFSET: usize = 9;
const KZG_N_BLOBS_OFFSET: usize = 1;

#[derive(Drop, Serde, Debug)]
pub struct StarknetOsOutput {
    pub initial_root: felt252,
    pub final_root: felt252,
    pub prev_block_number: felt252,
    pub new_block_number: felt252,
    pub prev_block_hash: felt252,
    pub new_block_hash: felt252,
    pub os_program_hash: felt252,
    pub starknet_os_config_hash: felt252,
    pub use_kzg_da: felt252,
    pub full_output: felt252,
    pub messages_to_l1: Span<MessageToStarknet>,
    pub messages_to_l2: Span<MessageToAppchain>,
}

#[derive(Drop, Serde, Debug)]
pub struct MessageToStarknet {
    /// Appchain contract address sending the message.
    pub from_address: ContractAddress,
    /// Starknet contract address receiving the message.
    pub to_address: ContractAddress,
    /// Payload of the message.
    pub payload: Span<felt252>,
}

#[derive(Drop, Serde, Debug)]
pub struct MessageToAppchain {
    /// Starknet address sending the message.
    pub from_address: ContractAddress,
    /// Appchain address receiving the message.
    pub to_address: ContractAddress,
    /// Nonce.
    pub nonce: felt252,
    /// Function selector (with #[l1 handler] attribute).
    pub selector: felt252,
    /// Payload size.
    pub payload: Span<felt252>,
}

fn read_segment(ref input_iter: SpanIter<felt252>, segment_length: usize) -> Array<felt252> {
    let mut segment = array![];
    for _i in 0..segment_length {
        let x = input_iter.next();
        if x.is_none() {
            break;
        }
        segment.append(*(x.unwrap()));
    };
    return segment;
}

/// Custom deserialization function, inspired by
/// https://github.com/starkware-libs/cairo-lang/blob/8e11b8cc65ae1d0959328b1b4a40b92df8b58595/src/starkware/starknet/core/aggregator/output_parser.py.
///
/// This deserialization function is expecting a bootloaded Starknet OS output, where the first
/// three elements of the input are part of the bootloader header.
pub fn deserialize_os_output(ref input_iter: SpanIter<felt252>) -> StarknetOsOutput {
    // Skip the bootloader header, which is not relevant for the SNOS output.
    let _ = read_segment(ref input_iter, 3);
    let header = read_segment(ref input_iter, HEADER_SIZE);
    let use_kzg_da = header[USE_KZG_DA_OFFSET];
    let full_output = header[FULL_OUTPUT_OFFSET];
    let os_program_hash = header[OS_PROGRAM_HASH_OFFSET];

    // StarknetOS (SNOS) program is expected to be run without an aggregator program at the moment.
    // Once aggregator program is supported, this will need to be updated for a conditional branch
    // to verify that the aggregator program is allowed to be run (added via the configuration
    // component).
    assert!(os_program_hash.is_zero(), "Aggregator program is not supported yet");

    // Currently not supported by the appchain logic, but will be added in the future.
    assert!(use_kzg_da.is_zero(), "KZG DA is not supported yet");

    assert!(full_output.is_zero(), "Full output is not supported");

    let (messages_to_l1, messages_to_l2) = deserialize_messages(ref input_iter);

    StarknetOsOutput {
        initial_root: *header[PREVIOUS_MERKLE_UPDATE_OFFSET],
        final_root: *header[NEW_MERKLE_UPDATE_OFFSET],
        prev_block_number: *header[PREV_BLOCK_NUMBER_OFFSET],
        new_block_number: *header[NEW_BLOCK_NUMBER_OFFSET],
        prev_block_hash: *header[PREV_BLOCK_HASH_OFFSET],
        new_block_hash: *header[NEW_BLOCK_HASH_OFFSET],
        os_program_hash: *header[OS_PROGRAM_HASH_OFFSET],
        starknet_os_config_hash: *header[CONFIG_HASH_OFFSET],
        use_kzg_da: *use_kzg_da,
        full_output: *full_output,
        messages_to_l1: messages_to_l1,
        messages_to_l2: messages_to_l2,
    }
}

pub fn deserialize_messages(
    ref input_iter: SpanIter<felt252>,
) -> (Span<MessageToStarknet>, Span<MessageToAppchain>) {
    let n_messages_to_l1: usize = (*(input_iter.next().unwrap()))
        .try_into()
        .expect('Invalid n_messages_to_l1');
    let messages_to_l1 = read_segment(ref input_iter, n_messages_to_l1);
    let n_messages_to_l2: usize = (*(input_iter.next().unwrap()))
        .try_into()
        .expect('Invalid n_messages_to_l2');
    let mut messages_to_l2 = read_segment(ref input_iter, n_messages_to_l2);

    let mut iter_messages_to_l1 = messages_to_l1.span().into_iter();
    let messages_to_l1 = deserialize_messages_to_l1(ref iter_messages_to_l1);

    let mut iter_messages_to_l2 = messages_to_l2.span().into_iter();
    let messages_to_l2 = deserialize_messages_to_l2(ref iter_messages_to_l2);

    (messages_to_l1.span(), messages_to_l2.span())
}

fn deserialize_messages_to_l1(ref input_iter: SpanIter<felt252>) -> Array<MessageToStarknet> {
    let mut messages_to_starknet = array![];
    loop {
        let header = read_segment(ref input_iter, MESSAGE_TO_STARKNET_HEADER_SIZE);
        if header.len() < MESSAGE_TO_STARKNET_HEADER_SIZE {
            break;
        }
        let payload_size: usize = (*header[2]).try_into().expect('Invalid payload size');
        let mut payload = read_segment(ref input_iter, payload_size);
        let payload = payload.span();
        let from_address: ContractAddress = (*header[0]).try_into().expect('Invalid from address');
        let to_address: ContractAddress = (*header[1]).try_into().expect('Invalid to address');
        let message_to_starknet = MessageToStarknet { from_address, to_address, payload };
        messages_to_starknet.append(message_to_starknet);
    };
    return messages_to_starknet;
}

fn deserialize_messages_to_l2(ref input_iter: SpanIter<felt252>) -> Array<MessageToAppchain> {
    let mut messages_to_appchain = array![];
    loop {
        let header = read_segment(ref input_iter, MESSAGE_TO_APPCHAIN_HEADER_SIZE);
        if header.len() < MESSAGE_TO_APPCHAIN_HEADER_SIZE {
            break;
        }
        let payload_size: usize = (*header[4]).try_into().expect('Invalid payload size');
        let mut payload = read_segment(ref input_iter, payload_size);
        let payload = payload.span();
        let from_address: ContractAddress = (*header[0]).try_into().expect('Invalid from address');
        let to_address: ContractAddress = (*header[1]).try_into().expect('Invalid to address');
        let message_to_appchain = MessageToAppchain {
            from_address, to_address, nonce: *header[2], selector: *header[3], payload,
        };
        messages_to_appchain.append(message_to_appchain);
    };
    return messages_to_appchain;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected: "KZG DA is not supported yet")]
    fn test_deserialize_os_output_kzg_failure() {
        let mut input = array![];
        // Bootloader header.
        input.append(0);
        input.append(0);
        input.append(0);
        // SNOS output header.
        input.append('1');
        input.append('2');
        input.append('3');
        input.append('4');
        input.append('5');
        input.append('6');
        input.append(0);
        input.append('8');
        // use_kzg_da.
        input.append(1);
        // full_output.
        input.append(0);
        // messages_to_l1.
        input.append(0);
        // messages_to_l2.
        input.append(0);

        let mut input_iter = input.span().into_iter();
        let _os_output = deserialize_os_output(ref input_iter);
    }

    #[test]
    #[should_panic(expected: "Full output is not supported")]
    fn test_deserialize_os_output_full_output_failure() {
        let mut input = array![];
        // Bootloader header.
        input.append(0);
        input.append(0);
        input.append(0);
        // SNOS output header.
        input.append('1');
        input.append('2');
        input.append('3');
        input.append('4');
        input.append('5');
        input.append('6');
        input.append(0);
        input.append('8');
        // use_kzg_da.
        input.append(0);
        // full_output.
        input.append(1);
        // messages_to_l1.
        input.append(0);
        // messages_to_l2.
        input.append(0);

        let mut input_iter = input.span().into_iter();
        let _os_output = deserialize_os_output(ref input_iter);
    }

    #[test]
    #[should_panic(expected: "Aggregator program is not supported yet")]
    fn test_deserialize_os_output_aggregator_program_failure() {
        let mut input = array![];
        // Bootloader header.
        input.append(0);
        input.append(0);
        input.append(0);
        // SNOS output header.
        input.append('1');
        input.append('2');
        input.append('3');
        input.append('4');
        input.append('5');
        input.append('6');
        input.append('7');
        input.append('8');
        // use_kzg_da.
        input.append(0);
        // full_output.
        input.append(0);
        // messages_to_l1.
        input.append(0);
        // messages_to_l2.
        input.append(0);

        let mut input_iter = input.span().into_iter();
        let _os_output = deserialize_os_output(ref input_iter);
    }

    #[test]
    fn test_deserialize_os_output_no_messages() {
        let mut input = array![];
        // Bootloader header.
        input.append(0);
        input.append(0);
        input.append(0);
        // SNOS output header.
        input.append('1');
        input.append('2');
        input.append('3');
        input.append('4');
        input.append('5');
        input.append('6');
        input.append(0);
        input.append('8');
        // use_kzg_da.
        input.append(0);
        // full_output.
        input.append(0);
        // messages_to_l1.
        input.append(0);
        // messages_to_l2.
        input.append(0);

        let mut input_iter = input.span().into_iter();
        let os_output = deserialize_os_output(ref input_iter);

        assert(os_output.initial_root == '1', 'initial_root mismatch');
        assert(os_output.final_root == '2', 'final_root mismatch');
        assert(os_output.prev_block_number == '3', 'prev_block_number mismatch');
        assert(os_output.new_block_number == '4', 'new_block_number mismatch');
        assert(os_output.prev_block_hash == '5', 'prev_block_hash mismatch');
        assert(os_output.new_block_hash == '6', 'new_block_hash mismatch');
        assert(os_output.os_program_hash == 0, 'os_program_hash mismatch');
        assert(os_output.starknet_os_config_hash == '8', 'snos config hash mismatch');
        assert(os_output.use_kzg_da == 0, 'use_kzg_da mismatch');
        assert(os_output.full_output == 0, 'full_output mismatch');
        assert(os_output.messages_to_l1.len() == 0, 'messages_to_l1 should be empty');
        assert(os_output.messages_to_l2.len() == 0, 'messages_to_l2 should be empty');
    }

    #[test]
    fn test_deserialize_os_output_with_messages() {
        let mut input = array![];
        // Bootloader header.
        input.append(0);
        input.append(0);
        input.append(0);
        // SNOS output header.
        input.append('1');
        input.append('2');
        input.append('3');
        input.append('4');
        input.append('5');
        input.append('6');
        input.append(0);
        input.append('8');
        // use_kzg_da.
        input.append(0);
        // full_output.
        input.append(0);

        // Add 1 message to L1 (segment length).
        input.append(5);
        // L1 message header.
        input.append('from_l1');
        input.append('to_l1');
        // Payload size and content.
        input.append(2);
        input.append('payload1');
        input.append('payload2');

        // Add 1 message to L2 (segment length).
        input.append(7);
        // L2 message header.
        input.append('from_l2');
        input.append('to_l2');
        input.append('nonce');
        input.append('selector');
        // Payload size and content.
        input.append(2);
        input.append('payload3');
        input.append('payload4');

        let mut input_iter = input.span().into_iter();
        let os_output = deserialize_os_output(ref input_iter);

        assert(os_output.messages_to_l1.len() == 1, 'should have 1 L1 message');
        assert(os_output.messages_to_l2.len() == 1, 'should have 1 L2 message');

        let l1_msg = os_output.messages_to_l1.at(0);
        assert((*l1_msg.from_address).into() == 'from_l1', 'L1 from_address mismatch');
        assert((*l1_msg.to_address).into() == 'to_l1', 'L1 to_address mismatch');
        assert((*l1_msg.payload).len() == 2, 'L1 payload length mismatch');
        assert(*(*l1_msg.payload).at(0) == 'payload1', 'L1 payload[0] mismatch');
        assert(*(*l1_msg.payload).at(1) == 'payload2', 'L1 payload[1] mismatch');

        let l2_msg = os_output.messages_to_l2.at(0);
        assert((*l2_msg.from_address).into() == 'from_l2', 'L2 from_address mismatch');
        assert((*l2_msg.to_address).into() == 'to_l2', 'L2 to_address mismatch');
        assert(*l2_msg.nonce == 'nonce', 'L2 nonce mismatch');
        assert(*l2_msg.selector == 'selector', 'L2 selector mismatch');
        assert((*l2_msg.payload).len() == 2, 'L2 payload length mismatch');
        assert(*(*l2_msg.payload).at(0) == 'payload3', 'L2 payload[0] mismatch');
        assert(*(*l2_msg.payload).at(1) == 'payload4', 'L2 payload[1] mismatch');
    }
}
