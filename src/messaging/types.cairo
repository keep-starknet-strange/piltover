pub type MessageHash = felt252;
pub type Nonce = felt252;

#[derive(Serde, Drop, PartialEq)]
pub enum MessageToAppchainStatus {
    SealedOrNotSent, // sn->appc: The nonce is 0 for the message hash.
    Pending: Nonce, // sn->appc: The nonce > 0.
}

#[derive(Serde, Drop, PartialEq)]
pub enum MessageToStarknetStatus {
    NothingToConsume, // appc->sn: the ref count is 0.
    ReadyToConsume: felt252 // appc->sn: the ref count > 0.
}
