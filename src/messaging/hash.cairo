//! SPDX-License-Identifier: MIT
//!
//! Hash utilities.
use starknet::ContractAddress;

/// Computes the hash of a message that is sent from Starknet to the Appchain.
///
/// <https://github.com/starkware-libs/cairo-lang/blob/caba294d82eeeccc3d86a158adb8ba209bf2d8fc/src/starkware/starknet/solidity/StarknetMessaging.sol#L88>
///
/// # Arguments
///
/// * `from_address` - Contract address of the message sender on the Appchain.
/// * `to_address` - Contract address to send the message to on the Appchain.
/// * `selector` - The `l1_handler` function selector of the contract on the Appchain
///                to execute.
/// * `payload` - The message payload.
/// * `nonce` - Nonce of the message.
///
/// # Returns
///
/// The hash of the message from Starknet to the Appchain.
pub fn compute_message_hash_sn_to_appc(
    from_address: ContractAddress,
    to_address: ContractAddress,
    selector: felt252,
    payload: Span<felt252>,
    nonce: felt252,
) -> felt252 {
    let mut hash_data = array![
        from_address.into(), to_address.into(), nonce, selector, payload.len().into(),
    ];

    let mut i = 0_usize;
    loop {
        if i == payload.len() {
            break;
        }
        hash_data.append((*payload[i]));
        i += 1;
    };

    core::poseidon::poseidon_hash_span(hash_data.span())
}

/// Computes the hash of a message that is sent from the Appchain to Starknet.
///
/// <https://github.com/starkware-libs/cairo-lang/blob/caba294d82eeeccc3d86a158adb8ba209bf2d8fc/src/starkware/starknet/solidity/StarknetMessaging.sol#L137>
///
/// # Arguments
///
/// * `from_address` - Contract address of the message sender on the Appchain.
/// * `to_address` - Contract address to send the message to on the Appchain.
/// * `payload` - The message payload.
///
/// # Returns
///
/// The hash of the message from the Appchain to Starknet.
pub fn compute_message_hash_appc_to_sn(
    from_address: ContractAddress, to_address: ContractAddress, payload: Span<felt252>,
) -> felt252 {
    let mut hash_data: Array<felt252> = array![
        from_address.into(), to_address.into(), payload.len().into(),
    ];

    let mut i = 0_usize;
    loop {
        if i == payload.len() {
            break;
        }
        hash_data.append((*payload[i]));
        i += 1;
    };

    core::poseidon::poseidon_hash_span(hash_data.span())
}
