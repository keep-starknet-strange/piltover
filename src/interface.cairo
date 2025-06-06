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
    /// * `snos_output` - The StarknetOS state update output (bootloaded).
    /// * `layout_bridge_program_output` - The layout bridge proof output (bootloaded).
    fn update_state(ref self: T, snos_output: Span<felt252>, layout_bridge_output: Span<felt252>);
}
