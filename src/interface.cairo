//! SPDX-License-Identifier: MIT
//!
//! Interface for appchain settlement contract.

#[starknet::interface]
pub trait IAppchain<T> {
    /// Updates the state of the Appchain on Starknet,
    /// based on a proof of the StarknetOS that the state transition
    /// is valid.
    ///
    /// In the current state of the SN stack, the layout required by SNOS
    /// is not yet supported with the starknet onchain verifier (integrity).
    /// For this reason, two proofs are required:
    /// - A proof for SNOS execution.
    /// - A layout bridge proof, which uses a layout supported by the onchain verifier.
    ///
    /// # Arguments
    ///
    /// * `snos_output` - The StarknetOS state update output.
    /// * `program_output` - The layout bridge proof output.
    /// * `onchain_data_hash` - The hash of the onchain data.
    /// * `onchain_data_size` - The size of the onchain data.
    fn update_state(
        ref self: T,
        snos_output: Span<felt252>,
        program_output: Span<felt252>,
        onchain_data_hash: felt252,
        onchain_data_size: u256,
    );
}
