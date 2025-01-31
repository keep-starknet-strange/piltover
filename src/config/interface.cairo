//! SPDX-License-Identifier: MIT
//!
//! Interface for appchain settlement contract configuration.
use starknet::ContractAddress;

#[starknet::interface]
trait IConfig<T> {
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

    /// Sets the information of the program that generates
    /// layout bridge (Cairo verifier ran with StarknetOs proof, wrapped in bootloader).
    ///
    /// # Arguments
    ///
    /// * `program_hash` - The program hash of layout bridge program.
    /// * `config_hash` - The SNOS's config hash.
    /// * `snos_program_hash` - The SNOS program hash
    fn set_program_info(
        ref self: T, program_hash: felt252, config_hash: felt252, snos_program_hash: felt252
    );

    /// Gets the information of the program that generates the
    /// layout bridge (Cairo verifier ran with StarknetOs proof, wrapped in bootloader),
    /// and StarknetOs program hash
    ///
    /// # Returns
    ///
    /// The layout bridge program hash, SNOS configuration hash and SNOS program hash.
    fn get_program_info(self: @T) -> (felt252, felt252, felt252);

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
