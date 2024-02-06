use starknet::ContractAddress;

use snforge_std as snf;
use snforge_std::{CheatTarget, ContractClassTrait};

use piltover::config::{
    config_cpt, config_cpt::InternalTrait as ConfigInternal, IConfig, IConfigDispatcherTrait,
    IConfigDispatcher, config_mock
};

use super::constants as c;

fn deploy_mock() -> IConfigDispatcher {
    let contract = snf::declare('config_mock');
    let calldata = array![c::owner().into()];
    let contract_address = contract.deploy(@calldata).unwrap();
    IConfigDispatcher { contract_address }
}

#[test]
fn config_set_operator_ok() {
    let mock = deploy_mock();
    assert(mock.get_operator() == c::zero_addr(), 'expect 0 addr');

    snf::start_prank(CheatTarget::One(mock.contract_address), c::owner());

    mock.set_operator(c::bob());
    assert(mock.get_operator() == c::bob(), 'expect bob');
}

#[test]
#[should_panic(expected: ('Caller is not the owner',))]
fn config_set_operator_unauthorized() {
    let mock = deploy_mock();
    assert(mock.get_operator() == c::zero_addr(), 'expect 0 addr');

    mock.set_operator(c::bob());
    assert(mock.get_operator() == c::bob(), 'expect bob');
}

#[test]
fn config_set_program_info_ok() {
    let mock = deploy_mock();

    snf::start_prank(CheatTarget::One(mock.contract_address), c::owner());

    // Owner sets the info.
    mock.set_program_info(0x1, 0x2);
    assert(mock.get_program_info() == (0x1, 0x2), 'expect correct hashes');

    mock.set_operator(c::bob());

    // Bob as operator can also set the program info.
    snf::start_prank(CheatTarget::One(mock.contract_address), c::bob());
    mock.set_program_info(0x11, 0x22);

    assert(mock.get_program_info() == (0x11, 0x22), 'expect operator hashes');
}

#[test]
#[should_panic(expected: ('Config: not owner or operator',))]
fn config_set_program_info_unauthorized() {
    let mock = deploy_mock();

    // Bob is not an operator.
    snf::start_prank(CheatTarget::One(mock.contract_address), c::bob());
    mock.set_program_info(0x11, 0x22);
}

#[test]
fn config_set_facts_registry_ok() {
    let mock = deploy_mock();

    snf::start_prank(CheatTarget::One(mock.contract_address), c::owner());

    // Owner sets the address.
    mock.set_facts_registry(c::contract_a());
    assert(mock.get_facts_registry() == c::contract_a(), 'expect valid address');

    mock.set_operator(c::bob());

    // Bob as operator can also set the program info.
    snf::start_prank(CheatTarget::One(mock.contract_address), c::bob());
    mock.set_facts_registry(c::contract_b());

    assert(mock.get_facts_registry() == c::contract_b(), 'expect operator address');
}

#[test]
#[should_panic(expected: ('Config: not owner or operator',))]
fn config_set_facts_registry_unauthorized() {
    let mock = deploy_mock();

    // Bob is not an operator.
    snf::start_prank(CheatTarget::One(mock.contract_address), c::bob());
    mock.set_facts_registry(c::contract_a());
}
