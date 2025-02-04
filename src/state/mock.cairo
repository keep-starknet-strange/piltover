#[starknet::contract]
pub mod state_mock {
    use piltover::state::{state_cpt, state_cpt::InternalTrait as StateInternal};

    component!(path: state_cpt, storage: state, event: StateEvent);

    #[abi(embed_v0)]
    impl StateImpl = state_cpt::StateImpl<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        state: state_cpt::Storage,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        StateEvent: state_cpt::Event,
    }

    #[constructor]
    fn constructor(
        ref self: ContractState, state_root: felt252, block_number: felt252, block_hash: felt252,
    ) {
        self.state.initialize(state_root, block_number, block_hash);
    }
}
