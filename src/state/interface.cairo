//! SPDX-License-Identifier: MIT
//!
//! Interface for Appchain - Starknet state.
use piltover::snos_output::StarknetOsOutput;

#[starknet::interface]
pub trait IState<T> {
    /// Validates that the 'blockNumber' and the previous root are consistent with the
    /// current state and updates the state.
    ///
    /// # Arguments
    ///
    /// * `program_output` - The StarknetOS state update output.
    fn update(ref self: T, program_output: StarknetOsOutput);

    /// Gets the current state.
    ///
    /// # Returns
    ///
    /// The state root, the block number and the block hash.
    fn get_state(self: @T) -> (felt252, felt252, felt252);
}

#[cfg(feature: 'update_state_test')]
#[starknet::interface]
pub trait IStateTest<T> {
    /// Enables initialising the state of the contract at an arbitrary block
    #[cfg(feature: 'update_state_test')]
    fn set_state(ref self: T, state_root: felt252, block_number: felt252, block_hash: felt252);
}
