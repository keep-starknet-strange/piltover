use openzeppelin_testing::constants as c;
use piltover::config::{IConfigDispatcher, IConfigDispatcherTrait};
use snforge_std as snf;
use snforge_std::ContractClassTrait;

fn deploy_mock() -> IConfigDispatcher {
    let contract = match snf::declare("config_mock").unwrap() {
        snf::DeclareResult::Success(contract) => contract,
        _ => core::panic_with_felt252('AlreadyDeclared not expected'),
    };
    let calldata = array![c::OWNER().into()];
    let (contract_address, _) = contract.deploy(@calldata).unwrap();
    IConfigDispatcher { contract_address }
}

#[test]
fn config_register_operator_ok() {
    let mock = deploy_mock();
    assert(!mock.is_operator(c::OPERATOR()), 'expect not operator');

    snf::start_cheat_caller_address(mock.contract_address, c::OWNER());

    mock.register_operator(c::OPERATOR());
    assert(mock.is_operator(c::OPERATOR()), 'expect operator');
}

#[test]
fn config_register_multiple_operators_ok() {
    let mock = deploy_mock();
    assert(!mock.is_operator(c::OPERATOR()), 'expect not operator');
    assert(!mock.is_operator(c::OTHER()), 'expect not operator');

    snf::start_cheat_caller_address(mock.contract_address, c::OWNER());

    mock.register_operator(c::OPERATOR());
    mock.register_operator(c::OTHER());

    assert(mock.is_operator(c::OPERATOR()), 'expect operator');
    assert(mock.is_operator(c::OTHER()), 'expect operator');
}

#[test]
fn config_unregister_operator_ok() {
    let mock = deploy_mock();

    snf::start_cheat_caller_address(mock.contract_address, c::OWNER());

    mock.register_operator(c::OPERATOR());
    assert(mock.is_operator(c::OPERATOR()), 'expect operator');

    mock.unregister_operator(c::OPERATOR());
    assert(!mock.is_operator(c::OPERATOR()), 'expect not operator');
}

#[test]
fn config_unregister_multiple_operators_ok() {
    let mock = deploy_mock();

    snf::start_cheat_caller_address(mock.contract_address, c::OWNER());

    mock.register_operator(c::OPERATOR());
    mock.register_operator(c::OTHER());

    assert(mock.is_operator(c::OPERATOR()), 'expect operator');
    assert(mock.is_operator(c::OTHER()), 'expect operator');

    mock.unregister_operator(c::OPERATOR());
    mock.unregister_operator(c::OTHER());
    assert(!mock.is_operator(c::OPERATOR()), 'expect not operator');
    assert(!mock.is_operator(c::OTHER()), 'expect not operator');
}

#[test]
#[should_panic(expected: ('Caller is not the owner',))]
fn config_set_operator_unauthorized() {
    let mock = deploy_mock();
    assert(!mock.is_operator(c::OPERATOR()), 'expect 0 addr');

    mock.register_operator(c::OPERATOR());
    assert(mock.is_operator(c::OPERATOR()), 'expect operator');
}

#[test]
fn config_set_program_info_ok() {
    let mock = deploy_mock();

    snf::start_cheat_caller_address(mock.contract_address, c::OWNER());

    // Owner sets the info.
    mock.set_program_info(0x1, 0x2);
    assert(mock.get_program_info() == (0x1, 0x2), 'expect correct hashes');

    mock.register_operator(c::OPERATOR());

    // Operator can also set the program info.
    snf::start_cheat_caller_address(mock.contract_address, c::OPERATOR());
    mock.set_program_info(0x11, 0x22);

    assert(mock.get_program_info() == (0x11, 0x22), 'expect operator hashes');
}

#[test]
#[should_panic(expected: ('Config: not owner or operator',))]
fn config_set_program_info_unauthorized() {
    let mock = deploy_mock();

    snf::start_cheat_caller_address(mock.contract_address, c::OPERATOR());
    mock.set_program_info(0x11, 0x22);
}

#[test]
fn config_set_facts_registry_ok() {
    let mock = deploy_mock();

    snf::start_cheat_caller_address(mock.contract_address, c::OWNER());

    let facts_registry_address = starknet::contract_address_const::<0x123>();

    // Owner sets the address.
    mock.set_facts_registry(facts_registry_address);
    assert(mock.get_facts_registry() == facts_registry_address, 'expect valid address');

    mock.register_operator(c::OPERATOR());

    // Operator can also set the program info.
    snf::start_cheat_caller_address(mock.contract_address, c::OPERATOR());
    mock.set_facts_registry(c::OTHER());

    assert(mock.get_facts_registry() == c::OTHER(), 'expect other address');
}

#[test]
#[should_panic(expected: ('Config: not owner or operator',))]
fn config_set_facts_registry_unauthorized() {
    let mock = deploy_mock();

    let facts_registry_address = starknet::contract_address_const::<0x123>();

    // Other is not an operator.
    snf::start_cheat_caller_address(mock.contract_address, c::OTHER());
    mock.set_facts_registry(facts_registry_address);
}
