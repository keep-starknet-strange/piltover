use core::iter::IntoIterator;
use core::num::traits::Zero;
use openzeppelin_testing::constants as c;
#[cfg(feature: 'messaging_test')]
use piltover::messaging::IMessagingTest;
use piltover::messaging::{
    IMessaging, IMessagingDispatcher, IMessagingDispatcherTrait, hash, messaging_cpt,
    messaging_cpt::InternalTrait as MessagingInternal,
    messaging_cpt::{Event, MessageCanceled, MessageCancellationStarted, MessageSent},
    messaging_mock, types::{MessageToAppchainStatus, MessageToStarknetStatus},
};
use piltover::snos_output::{MessageToAppchain, MessageToStarknet, deserialize_messages};
use snforge_std as snf;
use snforge_std::{ContractClassTrait, EventSpy, EventSpyAssertionsTrait};

/// Deploys the mock with a specific cancellation delay.
fn deploy_mock_with_delay(cancellation_delay_secs: u64) -> (IMessagingDispatcher, EventSpy) {
    let contract = match snf::declare("messaging_mock").unwrap() {
        snf::DeclareResult::Success(contract) => contract,
        _ => core::panic_with_felt252('AlreadyDeclared not expected'),
    };
    let calldata = array![cancellation_delay_secs.into()];
    let (contract_address, _) = contract.deploy(@calldata).unwrap();

    let mut spy = snf::spy_events();

    (IMessagingDispatcher { contract_address }, spy)
}

/// Deploys the mock with default delay of 5 days.
fn deploy_mock() -> (IMessagingDispatcher, EventSpy) {
    // Default delay of 5 days.
    deploy_mock_with_delay(432000)
}

/// Returns the state of a component for testing. This must be used
/// to test internal functions or directly access the storage.
/// You can't spy event with this. Use deploy instead.
fn mock_state_testing() -> messaging_cpt::ComponentState<messaging_mock::ContractState> {
    messaging_cpt::component_state_for_testing()
}

/// Messages to starknet taken from mainnet:
/// <https://etherscan.io/tx/0xc1351dac330d1d66f98efc99d08d360c2e9bc3d772c09d228027fcded8f02458>.
fn get_message_to_starknet() -> MessageToStarknet {
    let mut felts = array![
        3256441166037631918262930812410838598500200462657642943867372734773841898370,
        993696174272377493693496825928908586134624850969,
        4,
        0,
        917360325178274450223200079540424150242461675748,
        300000000000000,
        0,
    ]
        .span();

    Serde::deserialize(ref felts).unwrap()
}

/// Messages to appchain taken from mainnet:
/// <https://etherscan.io/tx/0xc1351dac330d1d66f98efc99d08d360c2e9bc3d772c09d228027fcded8f02458>.
fn get_message_to_appchain() -> MessageToAppchain {
    let mut felts = array![
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

    Serde::deserialize(ref felts).unwrap()
}

/// Messages segments of SNOS output from mainnet:
/// <https://etherscan.io/tx/0xc1351dac330d1d66f98efc99d08d360c2e9bc3d772c09d228027fcded8f02458>.
fn get_messages_segments() -> Array<felt252> {
    array![
        // Header
        // 2308509181970242579758367820250590423941246005755407149765148974993919671160,
        // 1400208033537979038273563301858781654076731580449174584651309975875760580865,
        // 535683,
        // 2885081770536693045243577840233106668867645710434679941076039698247255604327,
        // 2590421891839256512113614983194993186457498815986333310670788206383913888162,

        // message to l1 (starknet in this context).
        7,
        3256441166037631918262930812410838598500200462657642943867372734773841898370,
        993696174272377493693496825928908586134624850969,
        4,
        0,
        917360325178274450223200079540424150242461675748,
        300000000000000,
        0,
        // message to l2 (appchain in this context).
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
}

#[test]
fn message_to_starknet_deser() {
    let m = get_message_to_starknet();

    assert(
        m
            .from_address
            .into() == 3256441166037631918262930812410838598500200462657642943867372734773841898370,
        'invalid from',
    );
    assert(m.to_address.into() == 993696174272377493693496825928908586134624850969, 'invalid to');
    assert(m.payload.len() == 4, 'invalid payoad len');
    assert((*m.payload[0]) == 0, 'invalid payoad 0');
    assert((*m.payload[1]) == 917360325178274450223200079540424150242461675748, 'invalid payoad 1');
    assert((*m.payload[2]) == 300000000000000, 'invalid payoad 2');
    assert((*m.payload[3]) == 0, 'invalid payoad 3');
}

#[test]
fn message_to_appchain_deser() {
    let m = get_message_to_appchain();

    assert(
        m.from_address.into() == 993696174272377493693496825928908586134624850969, 'invalid from',
    );
    assert(
        m
            .to_address
            .into() == 3256441166037631918262930812410838598500200462657642943867372734773841898370,
        'invalid to',
    );
    assert(m.nonce == 1629170, 'invalid nonce');
    assert(
        m.selector == 1285101517810983806491589552491143496277809242732141897358598292095611420389,
        'invalid selector',
    );

    assert(m.payload.len() == 3, 'invalid payoad len');
    assert(
        (*m
            .payload[0]) == 1905350129216923298156817020930524704572804705313566176282348575247442538663,
        'invalid payoad 0',
    );
    assert((*m.payload[1]) == 100000000000000000, 'invalid payoad 1');
    assert((*m.payload[2]) == 0, 'invalid payoad 2');
}

#[test]
fn send_message_ok() {
    let (mock, mut spy) = deploy_mock();

    let from = c::SPENDER();
    let to = c::RECIPIENT();
    let selector = selector!("func1");
    let payload = array![1, 2, 3];

    snf::start_cheat_caller_address(mock.contract_address, from);
    let (message_hash, nonce) = mock.send_message_to_appchain(to, selector, payload.span());

    assert(message_hash.is_non_zero(), 'invalid message hash');
    assert(nonce == 0, 'invalid nonce');

    let expected_event = MessageSent {
        message_hash, from, to, selector, nonce: nonce, payload: payload.span(),
    };

    spy.assert_emitted(@array![(mock.contract_address, Event::MessageSent(expected_event))]);
}

#[test]
fn sn_to_appchain_messages_ok() {
    let (mock, mut spy) = deploy_mock();

    let from = c::SPENDER();
    let to = c::RECIPIENT();
    let selector = selector!("func1");
    let payload = array![1, 2, 3];

    snf::start_cheat_caller_address(mock.contract_address, from);

    // Calculate the message_hash
    let message_hash = hash::compute_message_hash_sn_to_appc(
        from_address: from, to_address: to, :selector, payload: payload.span(), nonce: 0,
    );
    let is_pending_before = mock.sn_to_appchain_messages(message_hash);
    assert(is_pending_before == MessageToAppchainStatus::NotSent, 'Should not be pending before');

    snf::start_cheat_caller_address(from, from);
    let (message_hash, nonce) = mock.send_message_to_appchain(to, selector, payload.span());

    // Ensure the message is pending to consume
    let is_pending_after = mock.sn_to_appchain_messages(message_hash);
    assert(
        is_pending_after == MessageToAppchainStatus::Pending(nonce),
        'message not registered/pending',
    );

    let expected_event = MessageSent {
        message_hash, from, to, selector, nonce: nonce, payload: payload.span(),
    };

    spy.assert_emitted(@array![(mock.contract_address, Event::MessageSent(expected_event))]);
}

#[test]
fn start_cancellation_ok() {
    let (mock, mut spy) = deploy_mock();

    let from = c::SPENDER();
    let to = c::RECIPIENT();
    let selector = selector!("func1");
    let payload = array![1, 2, 3];

    snf::start_cheat_caller_address(mock.contract_address, from);
    let (message_hash, nonce) = mock.send_message_to_appchain(to, selector, payload.span());

    let message_hash_cancel = mock.start_message_cancellation(to, selector, payload.span(), nonce);
    assert(message_hash == message_hash_cancel, 'invalid message hash');

    let expected_sent = MessageSent {
        message_hash, from, to, selector, nonce: nonce, payload: payload.span(),
    };

    let expected_start_cancel = MessageCancellationStarted {
        message_hash, from, to, selector, nonce: nonce, payload: payload.span(),
    };

    spy
        .assert_emitted(
            @array![
                (mock.contract_address, Event::MessageSent(expected_sent)),
                // Only one cancellation started event is expected, as the message is already in its
                // cancellation process.
                (mock.contract_address, Event::MessageCancellationStarted(expected_start_cancel)),
            ],
        );
}

#[test]
#[should_panic(expected: ('CANCELLATION_ALREADY_REQUESTED',))]
fn start_cancellation_already_requested() {
    let (mock, _) = deploy_mock();

    let from = c::SPENDER();
    let to = c::RECIPIENT();
    let selector = selector!("func1");
    let payload = array![1, 2, 3];

    snf::start_cheat_caller_address(mock.contract_address, from);
    let (message_hash, nonce) = mock.send_message_to_appchain(to, selector, payload.span());

    let message_hash_cancel = mock.start_message_cancellation(to, selector, payload.span(), nonce);
    assert(message_hash == message_hash_cancel, 'invalid message hash');

    mock.start_message_cancellation(to, selector, payload.span(), nonce);
}

#[test]
#[should_panic(expected: ('NO_MESSAGE_TO_CANCEL',))]
fn start_cancellation_invalid_nonce() {
    let (mock, _) = deploy_mock();

    let to = c::RECIPIENT();
    let selector = selector!("func1");
    let payload = array![1, 2, 3];
    let nonce = 0x800000000000011000000000000000000000000000000000000000000000000;

    mock.start_message_cancellation(to, selector, payload.span(), nonce);
}

#[test]
#[should_panic(expected: ('NO_MESSAGE_TO_CANCEL',))]
fn start_cancellation_no_message_to_cancel() {
    let (mock, _) = deploy_mock();

    let to = c::RECIPIENT();
    let selector = selector!("func1");
    let payload = array![1, 2, 3];
    let nonce = 1;

    mock.start_message_cancellation(to, selector, payload.span(), nonce);
}

#[test]
#[should_panic(expected: ('CANCELLATION_ALREADY_DONE',))]
fn start_cancellation_already_done() {
    let delay_secs = 10;
    let (mock, mut spy) = deploy_mock_with_delay(delay_secs);

    let from = c::SPENDER();
    let to = c::RECIPIENT();
    let selector = selector!("func1");
    let payload = array![1, 2, 3];

    snf::start_cheat_caller_address(mock.contract_address, from);
    let (message_hash, nonce) = mock.send_message_to_appchain(to, selector, payload.span());

    // The block timestamp must not be 0 for the protocol to be valid.
    // This can't happen on-chain, but here it must be explicitely set greater than 0.
    snf::start_cheat_block_timestamp(mock.contract_address, 1);
    mock.start_message_cancellation(to, selector, payload.span(), nonce);

    snf::start_cheat_block_timestamp(mock.contract_address, delay_secs + 10);
    let message_hash_cancel = mock.cancel_message(to, selector, payload.span(), nonce);
    assert(message_hash_cancel == message_hash, 'invalid message hash');

    let expected_sent = MessageSent {
        message_hash, from, to, selector, nonce: nonce, payload: payload.span(),
    };

    let expected_start_cancel = MessageCancellationStarted {
        message_hash, from, to, selector, nonce: nonce, payload: payload.span(),
    };

    let expected_cancel = MessageCanceled {
        message_hash, from, to, selector, nonce: nonce, payload: payload.span(),
    };

    spy
        .assert_emitted(
            @array![
                (mock.contract_address, Event::MessageSent(expected_sent)),
                (mock.contract_address, Event::MessageCancellationStarted(expected_start_cancel)),
                (mock.contract_address, Event::MessageCanceled(expected_cancel)),
            ],
        );

    // Can't start the cancellation again since the message is already cancelled.
    mock.start_message_cancellation(to, selector, payload.span(), nonce);
}

#[test]
fn cancel_message_ok() {
    let delay_secs = 10;
    let (mock, mut spy) = deploy_mock_with_delay(delay_secs);

    let from = c::SPENDER();
    let to = c::RECIPIENT();
    let selector = selector!("func1");
    let payload = array![1, 2, 3];

    snf::start_cheat_caller_address(mock.contract_address, from);
    let (message_hash, nonce) = mock.send_message_to_appchain(to, selector, payload.span());

    // The block timestamp must not be 0 for the protocol to be valid.
    // This can't happen on-chain, but here it must be explicitely set greater than 0.
    snf::start_cheat_block_timestamp(mock.contract_address, 1);
    mock.start_message_cancellation(to, selector, payload.span(), nonce);

    snf::start_cheat_block_timestamp(mock.contract_address, delay_secs + 10);
    let message_hash_cancel = mock.cancel_message(to, selector, payload.span(), nonce);
    assert(message_hash_cancel == message_hash, 'invalid message hash');

    let expected_sent = MessageSent {
        message_hash, from, to, selector, nonce: nonce, payload: payload.span(),
    };

    let expected_start_cancel = MessageCancellationStarted {
        message_hash, from, to, selector, nonce: nonce, payload: payload.span(),
    };

    let expected_cancel = MessageCanceled {
        message_hash, from, to, selector, nonce: nonce, payload: payload.span(),
    };

    spy
        .assert_emitted(
            @array![
                (mock.contract_address, Event::MessageSent(expected_sent)),
                (mock.contract_address, Event::MessageCancellationStarted(expected_start_cancel)),
                (mock.contract_address, Event::MessageCanceled(expected_cancel)),
            ],
        );
}

#[test]
#[should_panic(expected: ('NO_MESSAGE_TO_CANCEL',))]
fn cancel_message_no_message_to_cancel() {
    let (mock, _) = deploy_mock();

    let to = c::RECIPIENT();
    let selector = selector!("func1");
    let payload = array![1, 2, 3];
    let nonce = 0;

    mock.cancel_message(to, selector, payload.span(), nonce + 1000);
}

#[test]
#[should_panic(expected: ('CANCELLATION_INVALID_TIMESTAMP',))]
fn cancel_message_cancellation_not_requested() {
    let delay_secs = 10;
    let (mock, _) = deploy_mock_with_delay(delay_secs);

    let from = c::SPENDER();
    let to = c::RECIPIENT();
    let selector = selector!("func1");
    let payload = array![1, 2, 3];

    snf::start_cheat_caller_address(mock.contract_address, from);
    let (_message_hash, nonce) = mock.send_message_to_appchain(to, selector, payload.span());

    // The block timestamp must not be 0 for the protocol to be valid, since the request time
    // will be invalid.
    snf::start_cheat_block_timestamp(mock.contract_address, 0);
    mock.start_message_cancellation(to, selector, payload.span(), nonce);

    snf::start_cheat_block_timestamp(mock.contract_address, delay_secs + 10);
    mock.cancel_message(to, selector, payload.span(), nonce);
}

#[test]
#[should_panic(expected: ('CANCELLATION_NOT_ALLOWED_YET',))]
fn cancel_message_cancellation_not_allowed_yet() {
    let delay_secs = 10;
    let (mock, _) = deploy_mock_with_delay(delay_secs);

    let from = c::SPENDER();
    let to = c::RECIPIENT();
    let selector = selector!("func1");
    let payload = array![1, 2, 3];

    snf::start_cheat_caller_address(mock.contract_address, from);
    let (_message_hash, nonce) = mock.send_message_to_appchain(to, selector, payload.span());

    snf::start_cheat_block_timestamp(mock.contract_address, 1);
    mock.start_message_cancellation(to, selector, payload.span(), nonce);

    snf::start_cheat_block_timestamp(mock.contract_address, 5);
    mock.cancel_message(to, selector, payload.span(), nonce);
}

#[test]
fn gather_messages_from_output_ok() {
    let mut felts = get_messages_segments().span().into_iter();
    let (messages_to_starknet, messages_to_appchain) = deserialize_messages(ref felts);

    assert(messages_to_starknet.len() == 1, 'missing msgs to sn');
    assert(messages_to_appchain.len() == 1, 'missing msgs to appc');
}

#[test]
fn process_messages_to_starknet_ok() {
    let mut mock = mock_state_testing();

    let m = get_message_to_starknet();
    let _message_hash = hash::compute_message_hash_appc_to_sn(
        m.from_address, m.to_address, m.payload,
    );

    let messages = array![m].span();

    mock.process_messages_to_starknet(messages);
}

#[test]
fn process_messages_to_appchain_ok() {
    let mut mock = mock_state_testing();

    let from = c::SPENDER();
    let to = c::RECIPIENT();
    let selector = selector!("func1");
    let payload = array![1, 2, 3];

    snf::start_cheat_caller_address(starknet::get_contract_address(), from);
    let (_message_hash, _nonce) = mock.send_message_to_appchain(to, selector, payload.span());

    let m = MessageToAppchain {
        from_address: from, to_address: to, nonce: 0, selector, payload: payload.span(),
    };

    let _message_hash = hash::compute_message_hash_sn_to_appc(
        m.from_address, m.to_address, m.selector, m.payload, m.nonce,
    );

    let messages = array![m].span();

    mock.process_messages_to_appchain(messages);
}

#[test]
#[should_panic(expected: ('INVALID_MESSAGE_TO_SEAL',))]
fn process_messages_to_appchain_no_seal() {
    let mut mock = mock_state_testing();

    let m = get_message_to_appchain();
    // 'm' was never sent, nothing to seal.

    let messages = array![m].span();

    mock.process_messages_to_appchain(messages);
}

#[test]
fn consume_message_from_appchain_ok() {
    let mut mock = mock_state_testing();

    let from = c::SPENDER();
    let to = starknet::get_contract_address();
    let payload = array![1, 2, 3].span();

    let messages = array![MessageToStarknet { from_address: from, to_address: to, payload }].span();

    mock.process_messages_to_starknet(messages);

    // Ensure the caller address inside the mock function is correctly set.
    snf::start_cheat_caller_address(to, to);
    mock.consume_message_from_appchain(from, payload);
}

#[test]
fn appchain_to_sn_messages_ok() {
    let mut mock = mock_state_testing();

    let from = c::SPENDER();
    let to = starknet::get_contract_address();
    let payload = array![1, 2, 3].span();

    let messages = array![MessageToStarknet { from_address: from, to_address: to, payload }].span();

    let message_hash = hash::compute_message_hash_appc_to_sn(from, to, payload);

    let previous_status = mock.appchain_to_sn_messages(message_hash);
    assert(previous_status == MessageToStarknetStatus::NothingToConsume, 'message already present');

    mock.process_messages_to_starknet(messages);

    // Ensure that message is available to consume
    let count_after = mock.appchain_to_sn_messages(message_hash);
    assert(count_after == MessageToStarknetStatus::ReadyToConsume(1), 'message not present');
}

#[cfg(feature: 'messaging_test')]
#[test]
fn appchain_to_sn_messages_hashes_test() {
    let mut mock = mock_state_testing();

    let from = c::SPENDER();
    let to = starknet::get_contract_address();
    let payload = array![1, 2, 3].span();

    let message_hash = hash::compute_message_hash_appc_to_sn(from, to, payload);

    let previous_status = mock.appchain_to_sn_messages(message_hash);
    assert(previous_status == MessageToStarknetStatus::NothingToConsume, 'message already present');

    mock.add_messages_hashes_from_appchain(array![message_hash].span());

    // Ensure that message is available to consume.
    let count_after = mock.appchain_to_sn_messages(message_hash);
    assert(count_after == MessageToStarknetStatus::ReadyToConsume(1), 'message not present');
}

#[test]
#[should_panic(expected: ('INVALID_MESSAGE_TO_CONSUME',))]
fn consume_message_from_appchain_invalid_to_consume() {
    let mut mock = mock_state_testing();

    let from = c::SPENDER();
    let to = starknet::get_contract_address();
    let payload = array![1, 2, 3].span();

    // Don't process the messages to starknet.

    // Ensure the caller address inside the mock function is correctly set.
    snf::start_cheat_caller_address(to, to);
    mock.consume_message_from_appchain(from, payload);
}
