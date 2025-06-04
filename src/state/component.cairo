//! SPDX-License-Identifier: MIT
//!
//! Appchain - Starknet state component.

/// Errors.
mod errors {
    pub const INVALID_BLOCK_NUMBER: felt252 = 'State: invalid block number';
    pub const INVALID_PREVIOUS_ROOT: felt252 = 'State: invalid previous root';
    pub const INVALID_PREVIOUS_BLOCK_NUMBER: felt252 = 'State: invalid prev block num';
    pub const INVALID_PREVIOUS_BLOCK_HASH: felt252 = 'State: invalid prev block hash';
}

/// State component.
#[starknet::component]
pub mod state_cpt {
    use piltover::snos_output::StarknetOsOutput;
    use piltover::state::interface::{IState, IStateUpdater};
    use starknet::storage::{StoragePointerReadAccess, StoragePointerWriteAccess};
    use super::errors;

    type StateRoot = felt252;
    type BlockNumber = felt252;
    type BlockHash = felt252;

    #[storage]
    pub struct Storage {
        pub state_root: StateRoot,
        pub block_number: BlockNumber,
        pub block_hash: BlockHash,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {}

    #[embeddable_as(StateImpl)]
    impl State<
        TContractState, +HasComponent<TContractState>,
    > of IState<ComponentState<TContractState>> {
        fn get_state(self: @ComponentState<TContractState>) -> (StateRoot, BlockNumber, BlockHash) {
            (self.state_root.read(), self.block_number.read(), self.block_hash.read())
        }
    }

    #[embeddable_as(StateUpdaterImpl)]
    impl StateUpdater<
        TContractState, +HasComponent<TContractState>,
    > of IStateUpdater<ComponentState<TContractState>> {
        fn update(ref self: ComponentState<TContractState>, program_output: StarknetOsOutput) {
            assert(
                self.block_number.read() == program_output.prev_block_number,
                errors::INVALID_BLOCK_NUMBER,
            );

            assert(
                self.block_hash.read() == program_output.prev_block_hash,
                errors::INVALID_PREVIOUS_BLOCK_HASH,
            );

            // fetl252 doesn't support PartialOrd, convert to u256 required to ensure
            // the new block number is greater than the current block number for a valid state
            // transition.
            let block_number_u256: u256 = self.block_number.read().into();
            let new_block_number_u256: u256 = program_output.new_block_number.into();
            let max_felt = 0x800000000000011000000000000000000000000000000000000000000000000;

            // For the first block, the contract is initialized with a genesis state, which
            // is a special case where the block number is set to the maximum felt value.
            if self.block_number.read() == max_felt {
                // Ensure the new block number is greater than or equal to 0 to be compatible
                // with the current and future SNOS implementations (where several blocks may be
                // processed in one execution, and it may apply to the genesis block).
                assert(new_block_number_u256 >= 0, errors::INVALID_BLOCK_NUMBER);
            } else {
                // In all other cases, the new block number must be greater than the current block
                // number. Same here, there is no direct assumption of the block number being
                // incremented by 1, event if it is the case in the current SNOS implementation.
                assert(new_block_number_u256 > block_number_u256, errors::INVALID_BLOCK_NUMBER);
            }

            self.block_number.write(program_output.new_block_number);
            self.block_hash.write(program_output.new_block_hash);

            assert(
                self.state_root.read() == program_output.initial_root,
                errors::INVALID_PREVIOUS_ROOT,
            );

            self.state_root.write(program_output.final_root);
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
