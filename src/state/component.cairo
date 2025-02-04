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
        fn update(ref self: ComponentState<TContractState>, program_output: StarknetOsOutput) {
            self.check_prev_block_number(@program_output);

            // Check the blockNumber first as the error is less ambiguous then
            // INVALID_PREVIOUS_ROOT.
            self.block_number.write(self.block_number.read() + 1);
            assert(
                self.block_number.read() == program_output.new_block_number,
                errors::INVALID_BLOCK_NUMBER,
            );

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

        ///  Validates that the previous block number that appears in the proof is the
        /// current block number in the state.
        fn check_prev_block_number(
            self: @ComponentState<TContractState>, program_output: @StarknetOsOutput,
        ) {
            let mut expected_prev_block_number: felt252 = self.block_number.read();
            let prev_state_root: felt252 = self.state_root.read();
            let prev_block_hash: felt252 = self.block_hash.read();

            // If block number and state root is 0, then we assume it hasn't been initialized yet
            // and the current program output belongs to the genesis block.
            if expected_prev_block_number == 0 && prev_state_root == 0 && prev_block_hash == 0 {
                // This is the maximum value for a felt252.
                //
                // See
                // https://github.com/starkware-libs/cairo-lang/blob/a86e92bfde9c171c0856d7b46580c66e004922f3/src/starkware/starknet/solidity/StarknetState.sol#L19-L39
                expected_prev_block_number =
                    0x800000000000011000000000000000000000000000000000000000000000000;
            }

            assert(
                expected_prev_block_number == *program_output.prev_block_number,
                errors::INVALID_PREVIOUS_BLOCK_NUMBER,
            );
        }
    }
}
