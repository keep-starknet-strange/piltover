## messaging

```rust
trait IMessaging<T> {
    fn send_message_to_appchain(
        ref self: T, to_address: ContractAddress, selector: felt252, payload: Span<felt252>
    ) -> (felt252, felt252);

    fn consume_message_from_appchain(
        ref self: T, from_address: ContractAddress, payload: Span<felt252>
    ) -> felt252;

    fn start_message_cancellation(
        ref self: T,
        to_address: ContractAddress,
        selector: felt252,
        payload: Span<felt252>,
        nonce: felt252,
    ) -> felt252;

    fn cancel_message(
        ref self: T,
        to_address: ContractAddress,
        selector: felt252,
        payload: Span<felt252>,
        nonce: felt252,
    ) -> felt252;
}
```
