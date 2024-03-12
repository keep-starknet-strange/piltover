//! SPDX-License-Identifier: MIT
//!
//! Interface for appchain settlement contract.
use starknet::ContractAddress;

#[starknet::interface]
trait IAppchain<T> {
    /// Updates the state of the Appchain on Starknet,
    /// based on a proof of the StarknetOS that the state transition
    /// is valid.
    ///
    /// # Arguments
    ///
    /// * `program_output` - The StarknetOS state update output.
    /// TODO: DA + facts.
    fn update_state(
        ref self: T,
        program_output: Span<felt252>,
        onchain_data_hash: felt252,
        onchain_data_size: u256
    );
}

