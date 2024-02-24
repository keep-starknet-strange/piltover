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
    /// TODO: Implement the logic to check for a successful state.
    fn update(ref self: T, program_output: Span<felt252>,);
}
