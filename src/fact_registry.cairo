#[derive(Drop, Serde)]
pub struct VerificationListElement {
    pub verification_hash: felt252,
    pub security_bits: u32,
    pub verifier_config: VerifierConfiguration,
}
#[derive(Drop, Serde)]
pub struct VerifierConfiguration {
    pub layout: felt252,
    pub hasher: felt252,
    pub stone_version: felt252,
    pub cairo_version: felt252,
}

#[starknet::interface]
pub trait IFactRegistry<T> {
    fn get_all_verifications_for_fact_hash(
        self: @T, fact: felt252,
    ) -> Array<VerificationListElement>;
}

#[derive(Copy, Drop, starknet::Store, Serde)]
pub struct IFactRegistryContract {
    pub contract_address: starknet::ContractAddress,
}

impl ISmartProofDispatcherImpl of IFactRegistry<IFactRegistryContract> {
    fn get_all_verifications_for_fact_hash(
        self: @IFactRegistryContract, fact: felt252,
    ) -> Array<VerificationListElement> {
        let mut __calldata__ = array![fact];

        let mut __dispatcher_return_data__ = starknet::syscalls::call_contract_syscall(
            *self.contract_address,
            selector!("get_all_verifications_for_fact_hash"),
            core::array::ArrayTrait::span(@__calldata__),
        );
        let mut __dispatcher_return_data__ = starknet::SyscallResultTrait::unwrap_syscall(
            __dispatcher_return_data__,
        );
        core::option::OptionTrait::expect(
            core::serde::Serde::<
                Array<VerificationListElement>,
            >::deserialize(ref __dispatcher_return_data__),
            'Returned data too short',
        )
    }
}

#[starknet::contract]
pub mod fact_registry_mock {
    #[storage]
    struct Storage {}

    #[abi(embed_v0)]
    impl FactRegistryImplMock of super::IFactRegistry<ContractState> {
        fn get_all_verifications_for_fact_hash(
            self: @ContractState, fact: felt252,
        ) -> Array<super::VerificationListElement> {
            let verification_list_element = super::VerificationListElement {
                verification_hash: 1,
                security_bits: 60,
                verifier_config: super::VerifierConfiguration {
                    layout: 1, hasher: 1, stone_version: 1, cairo_version: 1,
                },
            };
            array![verification_list_element]
        }
    }
}
