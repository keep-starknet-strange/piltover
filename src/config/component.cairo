//! SPDX-License-Identifier: MIT
//!
//! Base configuration for appchain contract.

/// Errors.
mod errors {
    const INVALID_CALLER: felt252 = 'Config: not owner or operator';
}

/// Configuration component.
///
/// Depends on `ownable` to ensure the configuration is
/// only editable by contract's owner.
#[starknet::component]
mod config_cpt {
    use starknet::ContractAddress;

    use openzeppelin::access::ownable::{
        OwnableComponent as ownable_cpt, OwnableComponent::InternalTrait as OwnableInternal,
        interface::IOwnable,
    };

    use piltover::config::interface::IConfig;

    use super::errors;

    #[storage]
    struct Storage {
        /// Appchain operator that is allowed to update the state.
        operator: ContractAddress,
        /// Program info (StarknetOS), with program hash and config hash.
        program_info: (felt252, felt252),
        /// Facts registry contract address.
        facts_registry: ContractAddress,
    }

    #[embeddable_as(ConfigImpl)]
    impl Config<
        TContractState,
        +HasComponent<TContractState>,
        impl Ownable: ownable_cpt::HasComponent<TContractState>,
        +Drop<TContractState>
    > of IConfig<ComponentState<TContractState>> {
        fn set_operator(ref self: ComponentState<TContractState>, address: ContractAddress) {
            get_dep_component!(self, Ownable).assert_only_owner();
            self.operator.write(address)
        }

        fn get_operator(self: @ComponentState<TContractState>) -> ContractAddress {
            self.operator.read()
        }

        fn set_program_info(
            ref self: ComponentState<TContractState>, program_hash: felt252, config_hash: felt252
        ) {
            self.assert_only_owner_or_operator();
            self.program_info.write((program_hash, config_hash))
        }

        fn get_program_info(self: @ComponentState<TContractState>) -> (felt252, felt252) {
            self.program_info.read()
        }

        fn set_facts_registry(ref self: ComponentState<TContractState>, address: ContractAddress) {
            self.assert_only_owner_or_operator();
            self.facts_registry.write(address)
        }

        fn get_facts_registry(self: @ComponentState<TContractState>) -> ContractAddress {
            self.facts_registry.read()
        }
    }

    #[generate_trait]
    impl InternalImpl<
        TContractState,
        +HasComponent<TContractState>,
        impl Ownable: ownable_cpt::HasComponent<TContractState>,
        +Drop<TContractState>
    > of InternalTrait<TContractState> {
        /// Asserts if the caller is the owner of the contract or
        /// the authorized operator. Reverts otherwise.
        fn assert_only_owner_or_operator(ref self: ComponentState<TContractState>) {
            assert(
                self.is_owner_or_operator(starknet::get_caller_address()), errors::INVALID_CALLER
            );
        }

        /// Verifies if the given address is the owner of the contract
        /// or the authorized operator.
        ///
        /// # Arguments
        ///
        /// * `address` - The contrat address to verify.
        fn is_owner_or_operator(
            ref self: ComponentState<TContractState>, address: ContractAddress
        ) -> bool {
            let owner = get_dep_component!(self, Ownable).owner();
            address == owner || address == self.operator.read()
        }
    }
}
