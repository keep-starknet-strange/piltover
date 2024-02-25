use piltover::state::{
    state_cpt, state_cpt::InternalTrait as StateInternal, IState, IStateDispatcher,
    IStateDispatcherTrait, state_mock,
};
use snforge_std as snf;
use snforge_std::{ContractClassTrait};

/// Deploys the mock with a specific state.
fn deploy_mock_with_state(
    global_root: felt252, block_number: felt252, block_hash: felt252,
) -> IStateDispatcher {
    let contract = snf::declare('state_mock');
    let calldata = array![global_root, block_number, block_hash];
    let contract_address = contract.deploy(@calldata).unwrap();
    IStateDispatcher { contract_address }
}

#[test]
fn state_update_ok() {
    let mock = deploy_mock_with_state(
        global_root: 'global_root', block_number: 1, block_hash: 'block_hash_1',
    );

    let mut valid_state_update = array![
        'global_root',
        'new_global_root',
        2,
        'block_hash_2',
        'config_hash', // Header.
        0, // appc to sn messages segment.
        0, // sn to appc messages segment.
    ]
        .span();

    mock.update(valid_state_update);

    let (global_root, block_number, block_hash) = mock.get_state();

    assert(global_root == 'new_global_root', 'invalid global root');
    assert(block_number == 2, 'invalid block number');
    assert(block_hash == 'block_hash_2', 'invalid block hash');
}

#[test]
#[should_panic(expected: ('State: invalid block number',))]
fn state_update_invalid_block_number() {
    let mock = deploy_mock_with_state(
        global_root: 'global_root', block_number: 1, block_hash: 'block_hash_1',
    );

    let mut invalid_state_update = array![
        'global_root',
        'new_global_root',
        99999,
        'block_hash_2',
        'config_hash', // Header.
        0, // appc to sn messages segment.
        0, // sn to appc messages segment.
    ]
        .span();

    mock.update(invalid_state_update);
}

#[test]
#[should_panic(expected: ('State: invalid previous root',))]
fn state_update_invalid_previous_root() {
    let mock = deploy_mock_with_state(
        global_root: 'global_root', block_number: 1, block_hash: 'block_hash_1',
    );

    let mut invalid_state_update = array![
        'invalid_global_root',
        'new_global_root',
        2,
        'block_hash_2',
        'config_hash', // Header.
        0, // appc to sn messages segment.
        0, // sn to appc messages segment.
    ]
        .span();

    mock.update(invalid_state_update);
}
