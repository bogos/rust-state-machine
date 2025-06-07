# Rust State Machine

This project implements a simple blockchain state machine in Rust, demonstrating fundamental blockchain concepts such as blocks, transactions, and modules (pallets).

## Features

- Block and transaction system
- Balance module
- Proof of Existence module
- Macro system for code generation
- Immutable state handling

## Requirements

- Rust (latest stable version)
- Cargo (included with Rust)

## Installation

Compile the project:
```bash
cargo build
```

## Project Structure

```
rust-state-machine/
├── src/
│   ├── main.rs           # Entry point and runtime configuration
│   ├── system.rs         # Base system module
│   ├── balances.rs       # Balance handling module
│   ├── proof_of_existence.rs  # Proof of existence module
│   └── support.rs        # Common utilities and types
├── macros/
│   └── src/             # Macro implementations
└── Cargo.toml           # Project configuration
```

## Execution

To run the project:

```bash
cargo run
```

The program will execute a series of example blocks demonstrating:
1. Balance transfers between accounts
2. Proof of existence claim creation
3. Claim revocation

## Usage Example

The example program creates three blocks that demonstrate the functionalities:

1. **Block 1**: Balance transfers
   - Alice transfers 30 units to Bob
   - Alice transfers 20 units to Charlie

2. **Block 2**: Claim creation
   - Alice creates a claim for "Hello, world!"
   - Bob attempts to create the same claim (will fail)

3. **Block 3**: Revocation and new claim
   - Alice revokes her claim
   - Bob creates a new claim for "Hello, world!"

## Development

### Adding a New Module

1. Create a new file in `src/` for your module
2. Implement the `Config` trait for your module
3. Create the `Pallet` structure with your logic
4. Use the `#[macros::call]` macro for functions you want to expose
5. Add your module to the `Runtime` in `main.rs`

### Module Example

```rust
#[macros::call]
impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self { /* initialization */ }
    }

    pub fn some_function(&mut self, caller: T::AccountId, /* other parameters */) -> DispatchResult {
        // Your logic here
        Ok(())
    }
}
```

## Testing

To run the tests:

```bash
cargo test
```
