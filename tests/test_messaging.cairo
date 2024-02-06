use starknet::ContractAddress;

use snforge_std as snf;
use snforge_std::{CheatTarget, ContractClassTrait};

use piltover::messaging::{
    messaging_cpt, messaging_cpt::InternalTrait as MessagingInternal, IMessaging,
    IMessagingDispatcherTrait, IMessagingDispatcher, messaging_mock
};

use super::constants as c;

fn deploy_mock() -> IMessagingDispatcher {
    let contract = snf::declare('messaging_mock');
    let calldata = array![];
    let contract_address = contract.deploy(@calldata).unwrap();
    IMessagingDispatcher { contract_address }
}
