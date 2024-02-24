//! SPDX-License-Identifier: MIT
//!
//! Appchain - Starknet state component.

/// Errors.
mod errors {}

/// State component.
#[starknet::component]
mod state_cpt {
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
        fn update(
            ref self: ComponentState<TContractState>, program_output: Span<felt252>,
        ) { // TODO
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
