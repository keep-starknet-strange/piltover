//! SPDX-License-Identifier: MIT
//!
//! Receives L1-attested SHARP facts through Starknet L1 -> L2 messaging.

#[derive(Copy, Drop, Serde, starknet::Store)]
pub struct AttestedFact {
    pub exists: bool,
    pub fact_program_hash: u256,
    pub state_transition_fact: u256,
    pub sharp_fact: u256,
}

#[starknet::interface]
pub trait IL1FactReceiver<T> {
    fn get_attested_fact(self: @T, update_id: felt252) -> AttestedFact;
    fn get_allowed_l1_sender(self: @T) -> felt252;
    fn set_allowed_l1_sender(ref self: T, allowed_l1_sender: felt252);
}

#[starknet::contract]
pub mod l1_fact_receiver {
    use piltover::l1_fact_receiver::{AttestedFact, IL1FactReceiver};
    use starknet::storage::{
        Map, StorageMapReadAccess, StorageMapWriteAccess, StoragePointerReadAccess,
        StoragePointerWriteAccess,
    };
    use starknet::{ContractAddress, get_caller_address};

    mod errors {
        pub const INVALID_L1_SENDER: felt252 = 'invalid l1 sender';
        pub const NOT_OWNER: felt252 = 'not owner';
    }

    #[storage]
    struct Storage {
        owner: ContractAddress,
        allowed_l1_sender: felt252,
        facts: Map<felt252, AttestedFact>,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {
        FactAttested: FactAttested,
    }

    #[derive(Drop, starknet::Event)]
    pub struct FactAttested {
        pub update_id: felt252,
        pub fact_program_hash: u256,
        pub state_transition_fact: u256,
        pub sharp_fact: u256,
    }

    #[constructor]
    fn constructor(ref self: ContractState, owner: ContractAddress, allowed_l1_sender: felt252) {
        self.owner.write(owner);
        self.allowed_l1_sender.write(allowed_l1_sender);
    }

    #[abi(embed_v0)]
    impl FactReceiverImpl of IL1FactReceiver<ContractState> {
        fn get_attested_fact(self: @ContractState, update_id: felt252) -> AttestedFact {
            self.facts.read(update_id)
        }

        fn get_allowed_l1_sender(self: @ContractState) -> felt252 {
            self.allowed_l1_sender.read()
        }

        fn set_allowed_l1_sender(ref self: ContractState, allowed_l1_sender: felt252) {
            assert(get_caller_address() == self.owner.read(), errors::NOT_OWNER);
            self.allowed_l1_sender.write(allowed_l1_sender);
        }
    }

    #[l1_handler]
    fn consume_attested_fact(
        ref self: ContractState,
        from_address: felt252,
        update_id: felt252,
        fact_program_hash: u256,
        state_transition_fact: u256,
        sharp_fact: u256,
    ) {
        assert(from_address == self.allowed_l1_sender.read(), errors::INVALID_L1_SENDER);

        let attested_fact = AttestedFact {
            exists: true, fact_program_hash, state_transition_fact, sharp_fact,
        };
        self.facts.write(update_id, attested_fact);
        self.emit(FactAttested { update_id, fact_program_hash, state_transition_fact, sharp_fact });
    }
}
