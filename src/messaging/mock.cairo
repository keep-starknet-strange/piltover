#[starknet::contract]
mod messaging_mock {
    use starknet::ContractAddress;

    use piltover::messaging::{
        messaging_cpt, messaging_cpt::InternalTrait as MessagingInternal, IMessaging
    };

    component!(path: messaging_cpt, storage: messaging, event: MessagingEvent);

    #[abi(embed_v0)]
    impl MessagingImpl = messaging_cpt::MessagingImpl<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        messaging: messaging_cpt::Storage
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        MessagingEvent: messaging_cpt::Event
    }

    #[constructor]
    fn constructor(ref self: ContractState) {}
}
