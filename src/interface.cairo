//! SPDX-License-Identifier: MIT
//!
//! Interface for appchain settlement contract.
use starknet::ContractAddress;

#[starknet::interface]
trait IAppchain<T> {
    /// Sets the operator that is in charge to push state updates.
    ///
    /// # Arguments
    ///
    /// * `address` - The operator account address.
    fn set_operator(ref self: T, address: ContractAddress);

    /// Gets the operator address.
    ///
    /// # Returns
    ///
    /// The operator's address.
    fn get_operator(self: @T) -> ContractAddress;

    /// Sets the information of the program that generates the
    /// state transition trace (namely StarknetOS).
    ///
    /// # Arguments
    ///
    /// * `program_hash` - The program hash.
    /// * `config_hash` - The program's config hash.
    fn set_program_info(ref self: T, program_hash: felt252, config_hash: felt252);

    /// Gets the information of the program that generates the
    /// state transition trace (namely StarknetOS).
    ///
    /// # Returns
    ///
    /// The program hash and it's configuration hash.
    fn get_program_info(self: @T) -> (felt252, felt252);

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
