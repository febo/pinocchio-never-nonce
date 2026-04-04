# `p-never-nonce`

Prevent the use of durable nonces in a transaction.

## Usage

There are 3 ways you can ensure a transaction does not include an advance nonce instruction.


### 1) Library

Add `p-never-nonce` as a dependency:

```
cargo add p-never-nonce
```

In your program add a call to `ensure_never_nonce`.

#### Example

```rust
use {
    p_never_nonce::ensure_never_nonce,
    pinocchio::{
        ProgramResult, entrypoint::{InstructionContext, MaybeAccount}, error::ProgramError,
        lazy_program_entrypoint, no_allocator, nostd_panic_handler,
    },
};

// Disable the memory allocator.
no_allocator!();
// Use a `no_std` panic handler.
nostd_panic_handler!();
// Process the input lazily.
lazy_program_entrypoint!(process_instruction);

pub fn process_instruction(mut context: InstructionContext) -> ProgramResult {
    let MaybeAccount::Account(instructions_sysvar) = context.next_account()? else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    // Fails if the first instruction is an advance nonce.
    ensure_never_nonce(&instructions_sysvar, ProgramError::InvalidArgument)
}
```

### 2) Cross-program invocation

Add `p-never-nonce` as a dependency with the `"cpi"` feature enabled:

```
cargo add p-never-nonce --features cpi
```

Invoke the "never-nonce" program from your program. The program id of the "never-nonce" program is: `pnn1ctaR1tbP7EGrcz3WtrJKknRxKmKqADztKY9C3YJ`.

#### Example

```rust
use {
    p_never_nonce::cpi::NeverNonce,
    pinocchio::{
        ProgramResult, entrypoint::{InstructionContext, MaybeAccount}, error::ProgramError,
        lazy_program_entrypoint, no_allocator, nostd_panic_handler,
    },
};

// Disable the memory allocator.
no_allocator!();
// Use a `no_std` panic handler.
nostd_panic_handler!();
// Process the input lazily.
lazy_program_entrypoint!(process_instruction);

pub fn process_instruction(mut context: InstructionContext) -> ProgramResult {
    let MaybeAccount::Account(instructions_sysvar) = context.next_account()? else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // The "never-nonce" CPI will return an error if the first instruction
    // is an advance nonce.
    NeverNonce {
        instructions_sysvar,
    }
    .invoke()
}
```

### 3) Instruction guard

Add `p-never-nonce` as a dependency with the `"instruction"` feature enabled:

```
cargo add p-never-nonce --features instruction
```

Add a "never-nonce" instruction to your transaction.

#### Example

```rust
let never_nonce_ix = p_never_nonce::instruction::never_nonce();

let tx = Transaction::new_signed_with_payer(
    &[some_other_ix, never_nonce_ix],
    Some(&payer.pubkey()),
    &[&payer],
    connection.get_latest_blockhash()?,
);
```

## License

The code is licensed under the [Apache License Version 2.0](LICENSE)
