//! SNOS output related types and variables.
//!
use core::num::traits::Zero;
use core::iter::Iterator;
use core::iter::IntoIterator;
use core::array::ArrayIter;
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
struct StarknetOsOutput {
    initial_root: felt252,
    final_root: felt252,
    prev_block_number: felt252,
    new_block_number: felt252,
    prev_block_hash: felt252,
    new_block_hash: felt252,
    os_program_hash: felt252,
    starknet_os_config_hash: felt252,
    use_kzg_da: felt252,
    full_output: felt252,
    messages_to_l1: Array<felt252>,
    messages_to_l2: Array<felt252>,
    contracts: Array<ContractChanges>,
    classes: Array<(felt252, felt252)>,
}
#[derive(Drop, Serde, Debug)]
struct ContractChanges {
    addr: felt252,
    nonce: felt252,
    class_hash: Option<felt252>,
    storage_changes: Array<(felt252, felt252)>,
}
fn read_segment(ref input_iter: ArrayIter<felt252>, segment_length: usize) -> Array<felt252> {
    let mut segment = array![];
    for _i in 0
        ..segment_length {
            let x = input_iter.next();
            if x.is_none() {
                break;
            }
            segment.append(x.unwrap());
        };
    return segment;
}
fn deserialize_os_output(ref input_iter: ArrayIter<felt252>) -> StarknetOsOutput {
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
    let (contracts, classes) = if use_kzg_da.is_zero() {
        (
            deserialize_contract_state(ref input_iter, *full_output),
            deserialize_contract_class_da_changes(ref input_iter, *full_output)
        )
    } else {
        (array![], array![])
    };
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
        contracts,
        classes,
    }
}
fn deserialize_messages(ref input_iter: ArrayIter<felt252>) -> (Array<felt252>, Array<felt252>) {
    let n_messages_to_l1: usize = input_iter
        .next()
        .unwrap()
        .try_into()
        .expect('Invalid n_messages_to_l1');
    let messages_to_l1 = read_segment(ref input_iter, n_messages_to_l1);
    let n_messages_to_l2: usize = input_iter
        .next()
        .unwrap()
        .try_into()
        .expect('Invalid n_messages_to_l2');
    let messages_to_l2 = read_segment(ref input_iter, n_messages_to_l2);
    (messages_to_l1, messages_to_l2)
}
fn deserialize_contract_state(
    ref input_iter: ArrayIter<felt252>, full_output: felt252
) -> Array<ContractChanges> {
    let output_n_updates: usize = input_iter
        .next()
        .unwrap()
        .try_into()
        .expect('Invalid output_n_updates');
    let mut contract_changes = array![];
    for _ in 0
        ..output_n_updates {
            contract_changes.append(deserialize_contract_state_inner(ref input_iter, full_output));
        };
    contract_changes
}
fn deserialize_contract_state_inner(
    ref input_iter: ArrayIter<felt252>, full_output: felt252
) -> ContractChanges {
    let bound: u256 = 18446744073709551616; // 2^64
    let addr = input_iter.next().unwrap();
    let value: u256 = input_iter.next().unwrap().try_into().expect('Invalid value');
    let new_value = value / bound;
    let n_actual_updates = value % bound;
    let was_class_updated = new_value / bound;
    let new_state_nonce = new_value % bound;
    let new_state_class_hash = if !full_output.is_zero() {
        let _prev_state_class_hash = input_iter.next().unwrap();
        Option::Some(input_iter.next().unwrap())
    } else {
        if !was_class_updated.is_zero() {
            Option::Some(input_iter.next().unwrap())
        } else {
            Option::None
        }
    };

    let n_actual_updates: usize = n_actual_updates.try_into().expect('Invalid n_actual_updates');
    let storage_changes = deserialize_da_changes(ref input_iter, n_actual_updates, full_output);
    ContractChanges {
        addr: addr,
        nonce: new_state_nonce.try_into().unwrap(),
        class_hash: new_state_class_hash,
        storage_changes
    }
}

fn deserialize_da_changes(
    ref input_iter: ArrayIter<felt252>, n_actual_updates: usize, full_output: felt252
) -> Array<(felt252, felt252)> {
    let mut storage_changes = array![];
    for _ in 0
        ..n_actual_updates {
            let key = input_iter.next().unwrap();
            if full_output.is_non_zero() {
                input_iter.next().unwrap();
            };
            let new_value = input_iter.next().unwrap();
            storage_changes.append((key, new_value));
        };
    storage_changes
}

fn deserialize_contract_class_da_changes(
    ref input_iter: ArrayIter<felt252>, full_output: felt252
) -> Array<(felt252, felt252)> {
    let output_n_updates: usize = input_iter
        .next()
        .unwrap()
        .try_into()
        .expect('Invalid output_n_updates');
    let mut contract_changes = array![];
    for _ in 0
        ..output_n_updates {
            let class_hash = input_iter.next().unwrap();
            if full_output.is_non_zero() {
                input_iter.next().unwrap();
            };
            let compiled_class_hash = input_iter.next().unwrap();
            contract_changes.append((class_hash, compiled_class_hash));
        };
    contract_changes
}
