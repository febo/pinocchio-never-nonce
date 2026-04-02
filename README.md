# `p-never-nonce`

Prevent the use of durable nonces in a transaction.

## Usage

```
cargo add p-never-nonce
```

In your program add a call to `ensure_never_nonce`:

```rust
use p_never_nonce::ensure_never_nonce;
use pinocchio::{
    ProgramResult,
    entrypoint::{InstructionContext, MaybeAccount},
    error::ProgramError,
    lazy_program_entrypoint, no_allocator, nostd_panic_handler,
};

no_allocator!();
nostd_panic_handler!();
lazy_program_entrypoint!(process_instruction);

pub fn process_instruction(mut context: InstructionContext) -> ProgramResult {
    let MaybeAccount::Account(instructions_sysvar) = context.next_account()? else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    ensure_never_nonce(&instructions_sysvar, ProgramError::InvalidArgument)
}
```

## License

The code is licensed under the [Apache License Version 2.0](LICENSE)
