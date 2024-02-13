//! Appchain testing.
//!
use openzeppelin::tests::utils::constants as c;
use piltover::config::{IConfig, IConfigDispatcherTrait, IConfigDispatcher};
use piltover::interface::{IAppchain, IAppchainDispatcherTrait, IAppchainDispatcher};
use piltover::messaging::{IMessaging, IMessagingDispatcherTrait, IMessagingDispatcher};
use piltover::snos_output::ProgramOutput;
use snforge_std as snf;
use snforge_std::{
    CheatTarget, ContractClassTrait, SpyOn, EventSpy, cheatcodes::events::EventAssertions,
};
use starknet::{ContractAddress, storage::StorageMemberAccessTrait};

/// Deploys the appchain contract.
fn deploy_with_owner(owner: felt252) -> (IAppchainDispatcher, EventSpy) {
    let contract = snf::declare('appchain');
    let calldata = array![owner];
    let contract_address = contract.deploy(@calldata).unwrap();

    let mut spy = snf::spy_events(SpyOn::One(contract_address));

    (IAppchainDispatcher { contract_address }, spy)
}

/// State update taken from mainnet:
/// <https://etherscan.io/tx/0xc1351dac330d1d66f98efc99d08d360c2e9bc3d772c09d228027fcded8f02458>.
fn get_state_update() -> Span<felt252> {
    let mut felts = array![
        // Header.
        2308509181970242579758367820250590423941246005755407149765148974993919671160,
        1400208033537979038273563301858781654076731580449174584651309975875760580865,
        535683,
        2885081770536693045243577840233106668867645710434679941076039698247255604327,
        2590421891839256512113614983194993186457498815986333310670788206383913888162,
        // appc to sn messages segment.
        7,
        3256441166037631918262930812410838598500200462657642943867372734773841898370,
        993696174272377493693496825928908586134624850969,
        4,
        0,
        917360325178274450223200079540424150242461675748,
        300000000000000,
        0,
        // sn to appc messages segment.
        8,
        993696174272377493693496825928908586134624850969,
        3256441166037631918262930812410838598500200462657642943867372734773841898370,
        1629170,
        1285101517810983806491589552491143496277809242732141897358598292095611420389,
        3,
        1905350129216923298156817020930524704572804705313566176282348575247442538663,
        100000000000000000,
        0,
    ]
        .span();

    felts
}

#[test]
fn snos_output_deser() {
    let mut felts = get_state_update();
    let output: ProgramOutput = Serde::deserialize(ref felts).unwrap();

    assert(
        output
            .prev_state_root == 2308509181970242579758367820250590423941246005755407149765148974993919671160,
        'invalid prev root'
    );
    assert(
        output
            .new_state_root == 1400208033537979038273563301858781654076731580449174584651309975875760580865,
        'invalid new root'
    );
    assert(output.block_number == 535683, 'invalid block number');
    assert(
        output
            .block_hash == 2885081770536693045243577840233106668867645710434679941076039698247255604327,
        'invalid block hash'
    );
    assert(
        output
            .config_hash == 2590421891839256512113614983194993186457498815986333310670788206383913888162,
        'invalid config hash'
    );

    assert(output.message_to_starknet_segment.len() == 7, 'invalid msg to sn len');
    assert(output.message_to_appchain_segment.len() == 8, 'invalid msg to appc len');
}

#[test]
fn constructor_ok() {
    let (_appchain, _spy) = deploy_with_owner(c::OWNER().into());
}

#[test]
fn appchain_owner_ok() {
    let (appchain, _spy) = deploy_with_owner(c::OWNER().into());

    let iconfig = IConfigDispatcher { contract_address: appchain.contract_address };

    snf::start_prank(CheatTarget::One(appchain.contract_address), c::OWNER());
    iconfig.set_program_info(0x11, 0x22);
}

#[test]
#[should_panic(expected: ('Config: not owner or operator',))]
fn appchain_owner_only() {
    let (appchain, _spy) = deploy_with_owner(c::OWNER().into());

    let iconfig = IConfigDispatcher { contract_address: appchain.contract_address };
    iconfig.set_program_info(0x11, 0x22);
}

#[test]
fn update_state_ok() {
    let (appchain, _spy) = deploy_with_owner(c::OWNER().into());

    let imsg = IMessagingDispatcher { contract_address: appchain.contract_address };

    let contract_sn = starknet::contract_address_const::<
        993696174272377493693496825928908586134624850969
    >();
    let contract_appc = starknet::contract_address_const::<
        3256441166037631918262930812410838598500200462657642943867372734773841898370
    >();
    let selector_appc =
        1285101517810983806491589552491143496277809242732141897358598292095611420389;
    let payload_sn_to_appc = array![
        1905350129216923298156817020930524704572804705313566176282348575247442538663,
        100000000000000000,
        0,
    ]
        .span();
    let payload_appc_to_sn = array![
        0, 917360325178274450223200079540424150242461675748, 300000000000000, 0,
    ]
        .span();

    // The state update contains a message to appchain, therefore, before
    // being sealed, it must be sent first.
    // The nonce must be adjusted to ensure the correct message to be sent.
    snf::store(
        appchain.contract_address, selector!("sn_to_appc_nonce"), array![1629170 - 1].span()
    );

    snf::start_prank(CheatTarget::One(appchain.contract_address), contract_sn);
    imsg.send_message_to_appchain(contract_appc, selector_appc, payload_sn_to_appc);

    // Updating the state will register the message to starknet ready to be consumed
    // and the message to appchain as sealed.
    let output = get_state_update();

    snf::start_prank(CheatTarget::One(appchain.contract_address), c::OWNER());
    appchain.update_state(output);

    snf::start_prank(CheatTarget::One(appchain.contract_address), contract_sn);
    imsg.consume_message_from_appchain(contract_appc, payload_appc_to_sn);
}
