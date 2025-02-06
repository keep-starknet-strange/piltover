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

#[derive(Drop, Serde, Debug)]
pub struct ContractChanges {
    pub addr: felt252,
    pub nonce: felt252,
    pub class_hash: Option<felt252>,
    pub storage_changes: Array<(felt252, felt252)>,
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
/// https://github.com/starkware-libs/cairo-lang/blob/8e11b8cc65ae1d0959328b1b4a40b92df8b58595/src/starkware/starknet/core/aggregator/output_parser.py
pub fn deserialize_os_output(ref input_iter: SpanIter<felt252>) -> StarknetOsOutput {
    let _ = read_segment(ref input_iter, 3);
    let header = read_segment(ref input_iter, HEADER_SIZE);
    let use_kzg_da = header[USE_KZG_DA_OFFSET];
    let full_output = header[FULL_OUTPUT_OFFSET];
    if use_kzg_da.is_non_zero() {
        let kzg_segment = read_segment(ref input_iter, 2);
        let n_blobs: usize = (*kzg_segment.at(KZG_N_BLOBS_OFFSET))
            .try_into()
            .expect('Invalid n_blobs');
        let _ = read_segment(ref input_iter, 2 * 2 * n_blobs);
    }
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
