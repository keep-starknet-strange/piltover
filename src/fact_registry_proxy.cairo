//! Proxy used by the Privily Satellite replay.
//!
//! Piltover keeps a configured "facts registry" address. This proxy preserves a
//! registry-style validation boundary while delegating Keccak fact checks to the
//! Herodotus Satellite with `is_mocked = false`.

#[starknet::interface]
pub trait ISatelliteKeccakFacts<T> {
    fn isKeccakFactHashValid(self: @T, fact_hash: u256, is_mocked: bool) -> bool;
}

#[starknet::interface]
pub trait IFactRegistryProxy<T> {
    fn is_fact_hash_valid_with_security(
        self: @T, fact_hash: u256, security_bits: u32,
    ) -> bool;
}

#[starknet::contract]
pub mod satellite_fact_registry_proxy {
    use piltover::fact_registry_proxy::{
        IFactRegistryProxy, ISatelliteKeccakFactsDispatcher, ISatelliteKeccakFactsDispatcherTrait,
    };
    use starknet::ContractAddress;
    use starknet::storage::{StoragePointerReadAccess, StoragePointerWriteAccess};

    #[storage]
    struct Storage {
        satellite: ContractAddress,
    }

    #[constructor]
    fn constructor(ref self: ContractState, satellite: ContractAddress) {
        self.satellite.write(satellite);
    }

    #[abi(embed_v0)]
    impl FactRegistryProxyImpl of IFactRegistryProxy<ContractState> {
        fn is_fact_hash_valid_with_security(
            self: @ContractState, fact_hash: u256, security_bits: u32,
        ) -> bool {
            let _ = security_bits;
            let satellite = ISatelliteKeccakFactsDispatcher {
                contract_address: self.satellite.read(),
            };
            satellite.isKeccakFactHashValid(fact_hash, false)
        }
    }
}
