use core::iter::IntoIterator;
use core::poseidon::{Poseidon, PoseidonImpl, HashStateImpl, poseidon_hash_span};
use core::result::ResultTrait;
//! Appchain testing.
//!
use openzeppelin_testing::constants as c;
use piltover::appchain::appchain::{Event, LogStateUpdate, LogStateTransitionFact};
use piltover::config::{IConfig, IConfigDispatcherTrait, IConfigDispatcher};
use piltover::fact_registry::{
    IFactRegistryDispatcher, IFactRegistryDispatcherTrait, fact_registry_mock
};
use piltover::interface::{IAppchain, IAppchainDispatcherTrait, IAppchainDispatcher};
use piltover::messaging::{IMessaging, IMessagingDispatcherTrait, IMessagingDispatcher};
use piltover::snos_output::{StarknetOsOutput, deserialize_os_output};
use snforge_std as snf;
use snforge_std::{ContractClassTrait, EventSpy, EventSpyAssertionsTrait};
use starknet::ContractAddress;

/// Deploys the appchain contract.
fn deploy_with_owner(owner: felt252) -> (IAppchainDispatcher, EventSpy) {
    let contract = snf::declare("appchain").unwrap();
    let calldata = array![owner, 0, 0, 0];
    let (contract_address, _) = contract.deploy(@calldata).unwrap();

    let mut spy = snf::spy_events();

    (IAppchainDispatcher { contract_address }, spy)
}

/// Deploys the appchain contract.
fn deploy_with_owner_and_state(
    owner: felt252, state_root: felt252, block_number: felt252, block_hash: felt252,
) -> (IAppchainDispatcher, EventSpy) {
    let contract = snf::declare("appchain").unwrap();
    let calldata = array![owner, state_root, block_number, block_hash];
    let (contract_address, _) = contract.deploy(@calldata).unwrap();

    let mut spy = snf::spy_events();

    (IAppchainDispatcher { contract_address }, spy)
}

/// Deploys the fact registry mock contract.
fn deploy_fact_registry_mock() -> IFactRegistryDispatcher {
    let contract = snf::declare("fact_registry_mock").unwrap();
    let (contract_address, _) = contract.deploy(@array![]).unwrap();
    IFactRegistryDispatcher { contract_address }
}

/// State update taken from mainnet:
/// <https://etherscan.io/tx/0xc1351dac330d1d66f98efc99d08d360c2e9bc3d772c09d228027fcded8f02458>.
fn get_state_update() -> Array<felt252> {
    let felts = array![
        1,
        2,
        3, //We add these three values to match the orginal way that snos output is sent to piltover
        1120029756675208924496185249815549700817638276364867982519015153297469423111,
        2251620073307221877548100532273969460343974267802546890497101472079704728659,
        97999,
        98000,
        531367489267323329537005801734709408229779133529698992357325410316912085961,
        1409866304326047723545512049056686384378458135319101511279962897250423318202,
        0,
        8868593919264901768958912247765226517850727970326290266005120699201631282,
        0,
        1,
        7,
        3256441166037631918262930812410838598500200462657642943867372734773841898370,
        993696174272377493693496825928908586134624850969,
        4,
        0,
        917360325178274450223200079540424150242461675748,
        300000000000000,
        0,
        8,
        993696174272377493693496825928908586134624850969,
        3256441166037631918262930812410838598500200462657642943867372734773841898370,
        1629170,
        1285101517810983806491589552491143496277809242732141897358598292095611420389,
        3,
        1905350129216923298156817020930524704572804705313566176282348575247442538663,
        100000000000000000,
        0,
        6,
        1,
        1,
        0,
        0,
        97990,
        0,
        1421899847879340096589325210043286649171466364502194593015082433117255221080,
        539811130998854911913368419557270514090341129164043584949402573087923126171,
        0,
        864168206232928110031491408115121152494276771844027082417862893190647730830,
        864168206232928110031491408115121152494276771844027082417862893190647730830,
        931882050818197104151576032658107634807803237374684585598735701845242787221,
        2,
        128662172379922508296467310900604878709271140867429774768717858724533392169,
        128662172379922508296467310900604878709271140867429774768717858724533392169,
        1224905988085919456474747761719257466987758683482742340824267505183421118586,
        0,
        241512468505471871266105123156688890601,
        1224905988085919456474747761719257466987758683482742340824267505183421118587,
        0,
        156244657262320997856355588425043590703,
        1843594293243437784757074475107366978095297949025468091716317559092489752458,
        2570492999720769153141660042528305312694272,
        1352728969551125079635810475883015946922963308901230687016395341018183857530,
        1352728969551125079635810475883015946922963308901230687016395341018183857530,
        2009894490435840142178314390393166646092438090257831307886760648929397478285,
        2,
        2115330844248268693627242987584098362538589374784454106852621151464840523339,
        2115330844248268693627242987584098362538589374784454106852621151464840523339,
        1614256279127588147803706130784883731696469375559695308496091798283508817150,
        1911948235149847007159,
        1911599417798406592733,
        2391257774930840040377368641019581761981250012793463501307151573516264315546,
        456607249509036544573417,
        456607598326387984987843,
        3342444174998114629834932650195498603964827041682183113621541646365277691425,
        0,
        700754364753995129749969601657926670551784737862886653008195701294418726041,
        700754364753995129749969601657926670551784737862886653008195701294418726041,
        0
    ];
    felts
}

fn get_output() -> Span<felt252> {
    let snos_output = get_state_update();
    let snos_output_hash = poseidon_hash_span(snos_output.span());
    let felts = array![1, 2, 3, 4, snos_output_hash];
    felts.span()
}

#[test]
fn snos_output_deser() {
    let mut felts = get_state_update().span().into_iter();
    let output: StarknetOsOutput = deserialize_os_output(ref felts);

    assert(
        output
            .initial_root == 1120029756675208924496185249815549700817638276364867982519015153297469423111,
        'invalid prev root'
    );
    assert(
        output
            .final_root == 2251620073307221877548100532273969460343974267802546890497101472079704728659,
        'invalid new root'
    );
    assert(output.new_block_number == 98000, 'invalid block number');
    assert(
        output
            .new_block_hash == 1409866304326047723545512049056686384378458135319101511279962897250423318202,
        'invalid block hash'
    );
    assert(output.os_program_hash == 0, 'invalid config hash');
    assert(output.messages_to_l1.len() == 1, 'invalid msg to sn len');
    assert(output.messages_to_l2.len() == 1, 'invalid msg to appc len');
}

#[test]
fn constructor_ok() {
    let (_appchain, _spy) = deploy_with_owner(c::OWNER().into());
}

#[test]
fn appchain_owner_ok() {
    let (appchain, _spy) = deploy_with_owner(c::OWNER().into());

    let iconfig = IConfigDispatcher { contract_address: appchain.contract_address };

    snf::start_cheat_caller_address(appchain.contract_address, c::OWNER());
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
    let (appchain, mut _spy) = deploy_with_owner_and_state(
        owner: c::OWNER().into(),
        state_root: 1120029756675208924496185249815549700817638276364867982519015153297469423111,
        block_number: 97999,
        block_hash: 0
    );

    let imsg = IMessagingDispatcher { contract_address: appchain.contract_address };
    let iconfig = IConfigDispatcher { contract_address: appchain.contract_address };

    let fact_registry_mock = deploy_fact_registry_mock();

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

    snf::start_cheat_caller_address(appchain.contract_address, c::OWNER());
    iconfig
        .set_program_info(
            program_hash: 0,
            config_hash: 8868593919264901768958912247765226517850727970326290266005120699201631282
        );
    iconfig.set_facts_registry(address: fact_registry_mock.contract_address);

    // The state update contains a message to appchain, therefore, before
    // being sealed, it must be sent first.
    // The nonce must be adjusted to ensure the correct message to be sent.
    snf::store(appchain.contract_address, selector!("sn_to_appc_nonce"), array![1629170].span());

    snf::start_cheat_caller_address(appchain.contract_address, contract_sn);
    imsg.send_message_to_appchain(contract_appc, selector_appc, payload_sn_to_appc);
    // Updating the state will register the message to starknet ready to be consumed
    // and the message to appchain as sealed.
    let snos_output = get_state_update();
    let output = get_output();
    let onchain_data_hash = 0x0;
    let onchain_data_size: u256 = 0;
    snf::start_cheat_caller_address(appchain.contract_address, c::OWNER());
    appchain.update_state(snos_output.span(), output, onchain_data_hash, onchain_data_size);

    let expected_log_state_update = LogStateUpdate {
        state_root: 2251620073307221877548100532273969460343974267802546890497101472079704728659,
        block_number: 98000,
        block_hash: 1409866304326047723545512049056686384378458135319101511279962897250423318202
    };

    let expected_state_transition_fact = LogStateTransitionFact {
        state_transition_fact: 95851155297023584948317010611313049107061888692476024929318287712294181461009
    };
    _spy
        .assert_emitted(
            @array![
                (appchain.contract_address, Event::LogStateUpdate(expected_log_state_update)),
                (
                    appchain.contract_address,
                    Event::LogStateTransitionFact(expected_state_transition_fact)
                )
            ]
        );
    snf::start_cheat_caller_address(appchain.contract_address, contract_sn);
    imsg.consume_message_from_appchain(contract_appc, payload_appc_to_sn);
}
