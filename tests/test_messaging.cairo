use piltover::messaging::{
    messaging_cpt, messaging_cpt::InternalTrait as MessagingInternal, IMessaging,
    IMessagingDispatcherTrait, IMessagingDispatcher, messaging_mock,
    messaging_cpt::{Event, MessageSent, MessageCancellationStarted, MessageCanceled},
};
use snforge_std as snf;
use snforge_std::{
    CheatTarget, ContractClassTrait, SpyOn, EventSpy, cheatcodes::events::EventAssertions
};
use starknet::ContractAddress;
use super::constants as c;

/// Deploys the mock with a specific cancellation delay.
fn deploy_mock_with_delay(cancellation_delay_secs: u64) -> (IMessagingDispatcher, EventSpy) {
    let contract = snf::declare('messaging_mock');
    let calldata = array![cancellation_delay_secs.into()];
    let contract_address = contract.deploy(@calldata).unwrap();

    let mut spy = snf::spy_events(SpyOn::One(contract_address));

    (IMessagingDispatcher { contract_address }, spy)
}

/// Deploys the mock with default delay of 5 days.
fn deploy_mock() -> (IMessagingDispatcher, EventSpy) {
    // Default delay of 5 days.
    deploy_mock_with_delay(432000)
}

// TODO: test consume message once the state update can increment the refcount of a message.

#[test]
fn send_message_ok() {
    let (mock, mut spy) = deploy_mock();

    let from = c::contract_a();
    let to = c::contract_b();
    let selector = selector!("func1");
    let payload = array![1, 2, 3];

    snf::start_prank(CheatTarget::One(mock.contract_address), c::contract_a());
    let (message_hash, nonce) = mock.send_message_to_appchain(to, selector, payload.span());

    assert(message_hash.is_non_zero(), 'invalid message hash');
    assert(nonce == 1, 'invalid nonce');

    let expected_event = MessageSent {
        message_hash, from, to, selector, nonce: nonce, payload: payload.span(),
    };

    spy.assert_emitted(@array![(mock.contract_address, Event::MessageSent(expected_event))]);
}

#[test]
fn start_cancellation_ok() {
    let (mock, mut spy) = deploy_mock();

    let from = c::contract_a();
    let to = c::contract_b();
    let selector = selector!("func1");
    let payload = array![1, 2, 3];

    snf::start_prank(CheatTarget::One(mock.contract_address), c::contract_a());
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
                (mock.contract_address, Event::MessageCancellationStarted(expected_start_cancel))
            ]
        );
}

#[test]
#[should_panic(expected: ('INVALID_NONCE',))]
fn start_cancellation_invalid_nonce() {
    let (mock, _) = deploy_mock();

    let from = c::contract_a();
    let to = c::contract_b();
    let selector = selector!("func1");
    let payload = array![1, 2, 3];
    let nonce = 0;

    mock.start_message_cancellation(to, selector, payload.span(), nonce);
}

#[test]
#[should_panic(expected: ('NO_MESSAGE_TO_CANCEL',))]
fn start_cancellation_no_message_to_cancel() {
    let (mock, _) = deploy_mock();

    let from = c::contract_a();
    let to = c::contract_b();
    let selector = selector!("func1");
    let payload = array![1, 2, 3];
    let nonce = 1;

    mock.start_message_cancellation(to, selector, payload.span(), nonce);
}

#[test]
fn cancel_message_ok() {
    let delay_secs = 10;
    let (mock, mut spy) = deploy_mock_with_delay(delay_secs);

    let from = c::contract_a();
    let to = c::contract_b();
    let selector = selector!("func1");
    let payload = array![1, 2, 3];

    snf::start_prank(CheatTarget::One(mock.contract_address), c::contract_a());
    let (message_hash, nonce) = mock.send_message_to_appchain(to, selector, payload.span());

    // The block timestamp must not be 0 for the protocol to be valid.
    // This can't happen on-chain, but here it must be explicitely set greater than 0.
    snf::start_warp(CheatTarget::One(mock.contract_address), 1);
    mock.start_message_cancellation(to, selector, payload.span(), nonce);

    snf::start_warp(CheatTarget::One(mock.contract_address), delay_secs + 10);
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
                (mock.contract_address, Event::MessageCanceled(expected_cancel))
            ]
        );
}

#[test]
#[should_panic(expected: ('NO_MESSAGE_TO_CANCEL',))]
fn cancel_message_no_message_to_cancel() {
    let (mock, _) = deploy_mock();

    let to = c::contract_b();
    let selector = selector!("func1");
    let payload = array![1, 2, 3];
    let nonce = 1;

    mock.cancel_message(to, selector, payload.span(), nonce + 1000);
}

#[test]
#[should_panic(expected: ('CANCELLATION_NOT_REQUESTED',))]
fn cancel_message_cancellation_not_requested() {
    let delay_secs = 10;
    let (mock, _) = deploy_mock_with_delay(delay_secs);

    let from = c::contract_a();
    let to = c::contract_b();
    let selector = selector!("func1");
    let payload = array![1, 2, 3];

    snf::start_prank(CheatTarget::One(mock.contract_address), c::contract_a());
    let (message_hash, nonce) = mock.send_message_to_appchain(to, selector, payload.span());

    // Don't start the cancellation.

    snf::start_warp(CheatTarget::One(mock.contract_address), delay_secs + 10);
    mock.cancel_message(to, selector, payload.span(), nonce);
}

#[test]
#[should_panic(expected: ('CANCELLATION_NOT_ALLOWED_YET',))]
fn cancel_message_cancellation_not_allowed_yet() {
    let delay_secs = 10;
    let (mock, _) = deploy_mock_with_delay(delay_secs);

    let from = c::contract_a();
    let to = c::contract_b();
    let selector = selector!("func1");
    let payload = array![1, 2, 3];

    snf::start_prank(CheatTarget::One(mock.contract_address), c::contract_a());
    let (message_hash, nonce) = mock.send_message_to_appchain(to, selector, payload.span());

    snf::start_warp(CheatTarget::One(mock.contract_address), 1);
    mock.start_message_cancellation(to, selector, payload.span(), nonce);

    snf::start_warp(CheatTarget::One(mock.contract_address), 5);
    mock.cancel_message(to, selector, payload.span(), nonce);
}
