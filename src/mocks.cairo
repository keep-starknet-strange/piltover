//! SPDX-License-Identifier: MIT
//!
//! Mocks.

/// A fact registry mock to be used while Herodotus completes its implementation.
/// see : https://github.com/HerodotusDev/cairo-verifier/blob/keccak-fact-registry/
#[starknet::interface]
trait IFactRegistryMock<TContractState> {
    fn is_valid(self: @TContractState, fact: u256) -> bool;
}

#[starknet::contract]
mod fact_registry_mock {
    #[storage]
    struct Storage {}

    #[abi(embed_v0)]
    impl FactRegistryImplMock of super::IFactRegistryMock<ContractState> {
        fn is_valid(self: @ContractState, fact: u256) -> bool {
            true
        }
    }
}