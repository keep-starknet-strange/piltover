# Code Style

## Cairo

- Use `snake_case` for module name and not `PascalCase`.

- Don't import directly a function from a module.

```rust
// Prefer that:
let addr = starknet::contract_address_const::<0>();

// Instead of:
use starknet::contract_address_const;
let addr = contract_address_const::<0>();
```

- Document functions inside the trait, and add details if needed in the implementation.
