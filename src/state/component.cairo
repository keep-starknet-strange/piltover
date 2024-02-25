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
    use piltover::snos_output::ProgramOutput;
    use piltover::state::interface::IState;
    use super::errors;

    type GlobalRoot = felt252;
    type BlockNumber = felt252;
    type BlockHash = felt252;

    #[storage]
    struct Storage {
        global_root: GlobalRoot,
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
        fn update(ref self: ComponentState<TContractState>, program_output: Span<felt252>,) {
            let mut program_output = program_output;
            let program_output: ProgramOutput = Serde::deserialize(ref program_output).unwrap();

            // Check the blockNumber first as the error is less ambiguous then INVALID_PREVIOUS_ROOT.
            self.block_number.write(self.block_number.read() + 1);
            assert(
                self.block_number.read() == program_output.block_number,
                errors::INVALID_BLOCK_NUMBER
            );

            self.block_hash.write(program_output.block_hash);

            assert(
                self.global_root.read() == program_output.prev_state_root,
                errors::INVALID_PREVIOUS_ROOT
            );

            self.global_root.write(program_output.new_state_root);
        }

        fn get_state(
            self: @ComponentState<TContractState>
        ) -> (GlobalRoot, BlockNumber, BlockHash) {
            (self.global_root.read(), self.block_number.read(), self.block_hash.read())
        }
    }

    #[generate_trait]
    impl InternalImpl<
        TContractState, +HasComponent<TContractState>,
    > of InternalTrait<TContractState> {
        /// Initialized the messaging component.
        /// # Arguments
        ///
        /// * `global_root` - The global state root.
        /// * `block_number` - The current block number.
        /// * `block_hash` - The hash of the current block.
        fn initialize(
            ref self: ComponentState<TContractState>,
            global_root: GlobalRoot,
            block_number: BlockNumber,
            block_hash: BlockHash,
        ) {
            self.global_root.write(global_root);
            self.block_number.write(block_number);
            self.block_hash.write(block_hash);
        }
    }
}
