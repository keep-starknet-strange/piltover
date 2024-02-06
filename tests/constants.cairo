//! Test constants.
//!
use starknet::ContractAddress;

fn zero_addr() -> ContractAddress {
    starknet::contract_address_const::<0>()
}

fn contract_a() -> ContractAddress {
    starknet::contract_address_const::<'CONTRACT_A'>()
}

fn contract_b() -> ContractAddress {
    starknet::contract_address_const::<'CONTRACT_B'>()
}

fn owner() -> ContractAddress {
    starknet::contract_address_const::<'OWNER'>()
}

fn bob() -> ContractAddress {
    starknet::contract_address_const::<'BOB'>()
}

fn alice() -> ContractAddress {
    starknet::contract_address_const::<'ALICE'>()
}
