//! SNOS output related types and variables.
//!

/// Size of the header of the output of SNOS.
const HEADER_SIZE: usize = 5;
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
#[derive(Drop, Serde)]
struct ProgramOutput {
    /// The state commitment before this block.
    prev_state_root: felt252,
    /// The state commitment after this block.
    new_state_root: felt252,
    /// The number (height) of this block.
    block_number: felt252,
    /// The hash of this block.
    block_hash: felt252,
    /// The Starknet chain config hash
    config_hash: felt252,
    /// Currently, as the SNOS output is not using the cairo
    /// serialization, we can only have a felt252 array with the
    /// messages segments.
    message_to_starknet_segment: Array<felt252>,
    message_to_appchain_segment: Array<felt252>,
}
