pub type MessageHash = felt252;
pub type Nonce = felt252;

#[derive(Serde, Drop, PartialEq, starknet::Store, Default)]
pub enum MessageToAppchainStatus {
    #[default]
    NotSent,
    Sealed,
    Cancelled,
    Pending: Nonce // sn->appc: The nonce > 0.
}

#[derive(Serde, Drop, PartialEq)]
pub enum MessageToStarknetStatus {
    NothingToConsume, // appc->sn: the ref count is 0.
    ReadyToConsume: felt252 // appc->sn: the ref count > 0.
}
