//! SPDX-License-Identifier: MIT
//!
//!

mod Errors {
    const INVALID_ADDRESS: felt252 = 'Config: invalid address';
}

/// Appchain settlement contract on starknet.
#[starknet::contract]
mod appchain {
    use starknet::ContractAddress;
    use openzeppelin::access::ownable::{OwnableComponent as ownable_cpt, interface::IOwnable};

    use piltover::config::{config_cpt, config_cpt::InternalTrait as ConfigInternal, IConfig};

    component!(path: ownable_cpt, storage: ownable, event: OwnableEvent);
    component!(path: config_cpt, storage: config, event: ConfigEvent);

    #[abi(embed_v0)]
    impl ConfigImpl = config_cpt::ConfigImpl<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        ownable: ownable_cpt::Storage,
        #[substorage(v0)]
        config: config_cpt::Storage,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        OwnableEvent: ownable_cpt::Event,
        #[flat]
        ConfigEvent: config_cpt::Event,
    }

    /// Initializes the contract.
    ///
    /// # Arguments
    ///
    /// * `address` - The contract address of the owner.
    #[constructor]
    fn constructor(ref self: ContractState, owner: ContractAddress) {
        self.ownable.transfer_ownership(owner);

        assert(self.config.is_owner_or_operator(owner), 'bad');
    }
}
