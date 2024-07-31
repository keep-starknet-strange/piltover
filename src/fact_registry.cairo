#[starknet::interface]
trait IFactRegistry<T> {
    fn is_valid(self: @T, fact: felt252) -> bool;
}

#[derive(Copy, Drop, starknet::Store, Serde)]
struct IFactRegistryContract {
    pub contract_address: starknet::ContractAddress,
}

impl ISmartProofDispatcherImpl of IFactRegistry<IFactRegistryContract> {
    fn is_valid(self: @IFactRegistryContract, fact: felt252) -> bool {
        let mut __calldata__ = array![fact];

        let mut __dispatcher_return_data__ = starknet::syscalls::call_contract_syscall(
            *self.contract_address,
            selector!("is_valid"),
            core::array::ArrayTrait::span(@__calldata__),
        );
        let mut __dispatcher_return_data__ = starknet::SyscallResultTrait::unwrap_syscall(
            __dispatcher_return_data__
        );
        core::option::OptionTrait::expect(
            core::serde::Serde::<bool>::deserialize(ref __dispatcher_return_data__),
            'Returned data too short',
        )
    }
}
