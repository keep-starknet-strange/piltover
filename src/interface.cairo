//! SPDX-License-Identifier: MIT
//!
//! Interface for appchain settlement contract.

#[starknet::interface]
pub trait IAppchain<T> {
    /// Updates the state of the Appchain on Starknet,
    /// based on a proof of the StarknetOS that the state transition
    /// is valid.
    ///
    /// This L1-attested settlement path expects the SNOS proof to be verified
    /// on Ethereum first. The resulting SHARP fact is relayed back to Starknet
    /// and stored in the configured facts registry address, which is expected
    /// to implement `IL1FactReceiver`.
    ///
    /// # Arguments
    ///
    /// * `snos_output` - The raw StarknetOS state update output proven on L1.
    /// * `layout_bridge_program_output` - Unused in this settlement path.
    fn update_state(ref self: T, snos_output: Span<felt252>, layout_bridge_output: Span<felt252>);
}
