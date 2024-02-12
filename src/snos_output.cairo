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
