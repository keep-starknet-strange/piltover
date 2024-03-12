## config

```rust
trait IConfig<T> {
    fn set_operator(ref self: T, address: ContractAddress);

    fn get_operator(self: @T) -> ContractAddress;

    fn set_program_info(ref self: T, program_hash: felt252, config_hash: felt252);

    fn get_program_info(self: @T) -> (felt252, felt252);

    fn set_facts_registry(ref self: T, address: ContractAddress);

    fn get_facts_registry(self: @T) -> ContractAddress;
}
```
