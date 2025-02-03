use core::result::ResultTrait;
use piltover::snos_output::StarknetOsOutput;
use piltover::state::{IStateDispatcher, IStateDispatcherTrait};
use snforge_std as snf;
use snforge_std::ContractClassTrait;

/// Deploys the mock with a specific state.
fn deploy_mock_with_state(
    state_root: felt252, block_number: felt252, block_hash: felt252,
) -> IStateDispatcher {
    let contract = match snf::declare("state_mock").unwrap() {
        snf::DeclareResult::Success(contract) => contract,
        _ => core::panic_with_felt252('AlreadyDeclared not expected'),
    };
    let calldata = array![state_root, block_number, block_hash];
    let (contract_address, _) = contract.deploy(@calldata).unwrap();
    IStateDispatcher { contract_address }
}

#[test]
fn state_update_ok() {
    let mock = deploy_mock_with_state(state_root: 1, block_number: 1, block_hash: 1);
    let os_output = StarknetOsOutput {
        initial_root: 1,
        final_root: 2,
        prev_block_number: 1,
        new_block_number: 2,
        prev_block_hash: 1,
        new_block_hash: 2,
        os_program_hash: 1,
        starknet_os_config_hash: 1,
        use_kzg_da: 0,
        full_output: 0,
        messages_to_l1: array![].span(),
        messages_to_l2: array![].span(),
        contracts: array![],
        classes: array![],
    };
    mock.update(os_output);

    let (state_root, block_number, block_hash) = mock.get_state();

    assert(state_root == 2, 'invalid state root');
    assert(block_number == 2, 'invalid block number');
    assert(block_hash == 2, 'invalid block hash');
}

#[test]
#[should_panic(expected: ('State: invalid block number',))]
fn state_update_invalid_block_number() {
    let mock = deploy_mock_with_state(state_root: 1, block_number: 1, block_hash: 1);

    let os_output = StarknetOsOutput {
        initial_root: 1,
        final_root: 2,
        prev_block_number: 1,
        new_block_number: 'invalid_block_number',
        prev_block_hash: 1,
        new_block_hash: 2,
        os_program_hash: 1,
        starknet_os_config_hash: 1,
        use_kzg_da: 0,
        full_output: 0,
        messages_to_l1: array![].span(),
        messages_to_l2: array![].span(),
        contracts: array![],
        classes: array![],
    };

    mock.update(os_output);
}

#[test]
#[should_panic(expected: ('State: invalid previous root',))]
fn state_update_invalid_previous_root() {
    let mock = deploy_mock_with_state(state_root: 1, block_number: 1, block_hash: 1);

    let invalid_state_update = StarknetOsOutput {
        initial_root: 'invalid_previous_root',
        final_root: 2,
        prev_block_number: 1,
        new_block_number: 2,
        prev_block_hash: 1,
        new_block_hash: 2,
        os_program_hash: 1,
        starknet_os_config_hash: 1,
        use_kzg_da: 0,
        full_output: 0,
        messages_to_l1: array![].span(),
        messages_to_l2: array![].span(),
        contracts: array![],
        classes: array![],
    };

    mock.update(invalid_state_update);
}
