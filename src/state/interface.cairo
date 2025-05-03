//! SPDX-License-Identifier: MIT
//!
//! Interface for Appchain - Starknet state.
use piltover::snos_output::StarknetOsOutput;

#[starknet::interface]
pub trait IState<T> {
    /// Gets the current state.
    ///
    /// # Returns
    ///
    /// The state root, the block number and the block hash.
    fn get_state(self: @T) -> (felt252, felt252, felt252);
}

#[starknet::interface]
pub trait IStateUpdater<T> {
    fn update(ref self: T, program_output: StarknetOsOutput);
}
