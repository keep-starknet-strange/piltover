//! SPDX-License-Identifier: MIT
//!
//! Appchain - Starknet state component.

/// Errors.
mod errors {
    pub const INVALID_BLOCK_NUMBER: felt252 = 'State: invalid block number';
    pub const INVALID_PREVIOUS_ROOT: felt252 = 'State: invalid previous root';
    pub const INVALID_PREVIOUS_BLOCK_NUMBER: felt252 = 'State: invalid prev block num';
}

/// State component.
#[starknet::component]
pub mod state_cpt {
    use piltover::snos_output::StarknetOsOutput;
    use piltover::state::interface::IState;
    use starknet::storage::{
        MutableVecTrait, StorableStoragePointerReadAccess, StoragePointerReadAccess,
        StoragePointerWriteAccess, Vec, VecTrait,
    };
    // use starknet::storage::*;
    use super::errors;

    type StateRoot = felt252;
    type BlockNumber = felt252;
    type BlockHash = felt252;

    #[storage]
    pub struct Storage {
        pub state_root: StateRoot,
        pub block_number: BlockNumber,
        pub block_hash: BlockHash,
        pub snos_output: Vec<felt252>,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {}

    #[embeddable_as(StateImpl)]
    impl State<
        TContractState, +HasComponent<TContractState>,
    > of IState<ComponentState<TContractState>> {
        fn update(ref self: ComponentState<TContractState>, program_output: StarknetOsOutput) {
            assert(
                self.block_number.read() == program_output.prev_block_number,
                errors::INVALID_BLOCK_NUMBER,
            );

            self.block_number.write(program_output.new_block_number);
            self.block_hash.write(program_output.new_block_hash);

            assert(
                self.state_root.read() == program_output.initial_root,
                errors::INVALID_PREVIOUS_ROOT,
            );

            self.state_root.write(program_output.final_root);
        }

        fn get_state(self: @ComponentState<TContractState>) -> (StateRoot, BlockNumber, BlockHash) {
            (self.state_root.read(), self.block_number.read(), self.block_hash.read())
        }

        fn store_snos_output(
            ref self: ComponentState<TContractState>, snos_output: Span<felt252>, from_index: u64,
        ) {
            let mut new_length = snos_output.len();
            let final_index = from_index + new_length.into();
            for i in from_index..final_index {
                let storage_pointer = self.snos_output.at(i);
                storage_pointer.write(*snos_output[i.try_into().unwrap()]);
            }
        }

        fn get_snos_output(
            self: @ComponentState<TContractState>, till_index: u64,
        ) -> Array<felt252> {
            let mut output = array![];
            for i in 0..till_index {
                let value = self.snos_output.at(i).read();
                output.append(value);
            };
            output
        }
    }

    #[generate_trait]
    pub impl InternalImpl<
        TContractState, +HasComponent<TContractState>,
    > of InternalTrait<TContractState> {
        /// Initialized the messaging component.
        /// # Arguments
        ///
        /// * `state_root` - The state root.
        /// * `block_number` - The current block number.
        /// * `block_hash` - The hash of the current block.
        fn initialize(
            ref self: ComponentState<TContractState>,
            state_root: StateRoot,
            block_number: BlockNumber,
            block_hash: BlockHash,
        ) {
            self.state_root.write(state_root);
            self.block_number.write(block_number);
            self.block_hash.write(block_hash);
        }
    }
}
