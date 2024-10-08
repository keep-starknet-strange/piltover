//! SPDX-License-Identifier: MIT
//!
//! Appchain - Starknet state component.

/// Errors.
mod errors {
    const INVALID_BLOCK_NUMBER: felt252 = 'State: invalid block number';
    const INVALID_PREVIOUS_ROOT: felt252 = 'State: invalid previous root';
}

/// State component.
#[starknet::component]
mod state_cpt {
    use piltover::snos_output::StarknetOsOutput;
    use piltover::state::interface::IState;
    use super::errors;

    type StateRoot = felt252;
    type BlockNumber = felt252;
    type BlockHash = felt252;

    #[storage]
    struct Storage {
        state_root: StateRoot,
        block_number: BlockNumber,
        block_hash: BlockHash,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {}

    #[embeddable_as(StateImpl)]
    impl State<
        TContractState, +HasComponent<TContractState>,
    > of IState<ComponentState<TContractState>> {
        fn update(ref self: ComponentState<TContractState>, program_output: Span<felt252>,){
            let mut stripped_output = program_output.slice(1,program_output.len()/2);
            let program_output: StarknetOsOutput = Serde::deserialize(ref stripped_output).unwrap();

            // Check the blockNumber first as the error is less ambiguous then INVALID_PREVIOUS_ROOT.
            // self.block_number.write(self.block_number.read() + 1);
            self.block_number.write(program_output.new_block_number);
            // assert(
            //     self.block_number.read() == program_output.block_number,
            //     errors::INVALID_BLOCK_NUMBER
            // );

            // self.block_hash.write(program_output.block_hash);
            self.block_hash.write(program_output.new_block_hash);

            // assert(
            //     self.state_root.read() == program_output.prev_state_root,
            //     errors::INVALID_PREVIOUS_ROOT
            // );

            // self.state_root.write(program_output.new_state_root);
            self.state_root.write(program_output.final_root);
        }

        fn get_state(self: @ComponentState<TContractState>) -> (StateRoot, BlockNumber, BlockHash) {
            (self.state_root.read(), self.block_number.read(), self.block_hash.read())
        }
    }

    #[generate_trait]
    impl InternalImpl<
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
