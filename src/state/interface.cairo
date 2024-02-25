//! SPDX-License-Identifier: MIT
//!
//! Interface for Appchain - Starknet state.

#[starknet::interface]
trait IState<T> {
    /// Validates that the 'blockNumber' and the previous root are consistent with the
    /// current state and updates the state.
    ///
    /// # Arguments
    ///
    /// * `program_output` - The StarknetOS state update output.
    fn update(ref self: T, program_output: Span<felt252>,);

    /// Gets the current state.
    ///
    /// # Returns
    ///
    /// The state root, the block number and the block hash.
    fn get_state(self: @T) -> (felt252, felt252, felt252);
}
