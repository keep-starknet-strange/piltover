pub type MessageHash = felt252;
pub type Nonce = felt252;

#[derive(Serde, Drop, PartialEq, starknet::Store, Default)]
pub enum MessageToAppchainStatus {
    #[default]
    NotSent,
    Sealed,
    Cancelled,
    // sn->appc: The nonce > 0.
    // The nonce is not directly used by the appchain logic, but is used to track the status of the
    // message and is retrieved when status of a message is requested while pending.
    Pending: Nonce,
    Cancelling,
}

#[derive(Serde, Drop, PartialEq)]
pub enum MessageToStarknetStatus {
    NothingToConsume, // appc->sn: the ref count is 0.
    ReadyToConsume: felt252 // appc->sn: the ref count > 0.
}
