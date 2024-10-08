//! SNOS output related types and variables.
//!

/// Size of the header of the output of SNOS.
const HEADER_SIZE: usize = 10;
/// Size of the header of a message to Starknet, which is
/// right before the payload content.
const MESSAGE_TO_STARKNET_HEADER_SIZE: usize = 3;
/// Size of the header of a message to appchain, which is
/// right before the payload content.
const MESSAGE_TO_APPCHAIN_HEADER_SIZE: usize = 5;

/// The output of SNOS will change with Starknet update.
/// The current format is using the v0.13.0 format.
/// <https://github.com/starkware-libs/cairo-lang/blob/caba294d82eeeccc3d86a158adb8ba209bf2d8fc/src/starkware/starknet/core/os/output.cairo#L52>.
/// The names are taken from SNOS repository:
/// <https://github.com/keep-starknet-strange/snos/blob/ad9a7df5fdbb63c813db285346eb667e032762e0/src/io/output.rs#L17>.
#[derive(Drop,Serde)]
struct StarknetOsOutput{
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
#[derive(Drop,Serde)]
struct ContractChanges {
     addr: felt252,
     nonce: felt252,
     class_hash: felt252,
     storage_changes: Array<(felt252, felt252)>,
}