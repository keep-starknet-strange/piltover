//! SPDX-License-Identifier: MIT
//!
//! Interface for appchain settlement contract.

#[starknet::interface]
pub trait IAppchain<T> {
    /// Updates the state of the Appchain on Starknet,
    /// based on a proof of the StarknetOS that the state transition
    /// is valid.
    ///
    /// This settlement path expects a Keccak SHARP fact to be available in
    /// the configured facts registry address, which is expected to implement
    /// the Satellite fact registry interface.
    ///
    /// # Arguments
    ///
    /// * `snos_output` - The raw StarknetOS state update output.
    fn update_state(ref self: T, snos_output: Span<felt252>);
}
