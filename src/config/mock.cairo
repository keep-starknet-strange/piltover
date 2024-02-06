#[starknet::contract]
mod config_mock {
    use starknet::ContractAddress;

    use openzeppelin::access::ownable::{
        OwnableComponent as ownable_cpt, OwnableComponent::InternalTrait as OwnableInternal
    };

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
        config: config_cpt::Storage
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        OwnableEvent: ownable_cpt::Event,
        #[flat]
        ConfigEvent: config_cpt::Event
    }

    #[constructor]
    fn constructor(ref self: ContractState, owner: ContractAddress) {
        self.ownable.initializer(owner);
    }
}
