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

    /// Stores the SNOS output.
    ///
    /// # Arguments
    ///
    /// * `snos_output` - The SNOS output.
    fn store_snos_output(ref self: T, snos_output: Span<felt252>, from_index: u64);

    /// Gets the SNOS output.
    ///
    /// # Returns
    ///
    fn get_snos_output(self: @T, till_index: u64) -> Array<felt252>;
}
