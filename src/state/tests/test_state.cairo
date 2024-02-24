use piltover::state::{
    state_cpt, state_cpt::InternalTrait as StateInternal, IState, IStateDispatcher,
    IStateDispatcherTrait, state_mock,
};
use snforge_std as snf;
use snforge_std::{ContractClassTrait};

/// Deploys the mock with dummy values
fn deploy_mock() -> IStateDispatcher {
    let contract = snf::declare('state_mock');
    let calldata = array![0x0, 0x0, 0x0];
    let contract_address = contract.deploy(@calldata).unwrap();
    IStateDispatcher { contract_address }
}

#[test]
fn state_update_ok() {
    let _ = deploy_mock();
}
