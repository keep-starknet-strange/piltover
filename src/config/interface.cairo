//! SPDX-License-Identifier: MIT
//!
//! Interface for appchain settlement contract configuration.
use starknet::ContractAddress;

/// Information of the program verified onchain to apply the state transition.
///
/// In the current design, the StarknetOS (SNOS) is executed and proven first.
/// Since the layout used by SNOS is not verifiable onchain, a bridge layout
/// program is also executed on the proof generated from SNOS execution.
///
/// For this reason, the program info contains the hash of the SNOS program,
/// additionally to the `program_hash`, which in this case is the bridge layout program hash.
/// This ensures that the correct programs have been executed.
#[derive(starknet::Store, Drop, Serde, Copy, PartialEq)]
pub struct ProgramInfo {
    pub program_hash: felt252,
    pub config_hash: felt252,
    pub snos_program_hash: felt252,
}

#[starknet::interface]
pub trait IConfig<T> {
    /// Registers an operator that is in charge to push state updates.
    /// Multiple operators can be registered.
    /// # Arguments
    ///
    /// * `address` - The account address to register as an operator.
    fn register_operator(ref self: T, address: ContractAddress);

    /// Unregisters an operator.
    /// # Arguments
    ///
    /// * `address` - The operator account address to unregister.
    fn unregister_operator(ref self: T, address: ContractAddress);

    /// Verifies if the given address is an operator.
    /// # Arguments
    ///
    /// * `address` - The address to verify.
    ///
    /// # Returns
    ///
    /// True if the address is an operator, false otherwise.
    fn is_operator(self: @T, address: ContractAddress) -> bool;

    /// Sets the information of the program verified onchain to
    /// execute the state transition.
    ///
    /// # Arguments
    ///
    /// * `program_info` - The program information.
    fn set_program_info(ref self: T, program_info: ProgramInfo);

    /// Gets the information of the program verified onchain to
    /// execute the state transition.
    ///
    /// # Returns
    ///
    /// The program information.
    fn get_program_info(self: @T) -> ProgramInfo;

    /// Sets the facts registry contract address, which is already
    /// initialized with the verifier information.
    ///
    /// # Arguments
    ///
    /// * `address` - The facts registry contract's address.
    fn set_facts_registry(ref self: T, address: ContractAddress);

    /// Gets the facts registry contract address.
    ///
    /// # Returns
    ///
    /// The contract address of the facts registry.
    fn get_facts_registry(self: @T) -> ContractAddress;
}
