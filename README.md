# `p-never-nonce`

Prevent the use of durable nonces in a transaction.

<p>
    <a href="https://crates.io/crates/p-never-nonce"><img src="https://img.shields.io/crates/v/p-never-nonce?logo=rust" /></a>
    <a href="https://explorer.solana.com/address/pnn1ctaR1tbP7EGrcz3WtrJKknRxKmKqADztKY9C3YJ"><img src="https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Ffebo%2Fpinocchio-never-nonce%2Frefs%2Fheads%2Fmain%2Fprogram%2FCargo.toml&query=%24.package.version&label=program&logo=data:image/svg%2bxml;base64,PHN2ZyB3aWR0aD0iMzEzIiBoZWlnaHQ9IjI4MSIgdmlld0JveD0iMCAwIDMxMyAyODEiIGZpbGw9Im5vbmUiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CjxnIGNsaXAtcGF0aD0idXJsKCNjbGlwMF80NzZfMjQzMCkiPgo8cGF0aCBkPSJNMzExLjMxOCAyMjEuMDU3TDI1OS42NiAyNzYuNTU4QzI1OC41MzcgMjc3Ljc2NCAyNTcuMTc4IDI3OC43MjUgMjU1LjY2OSAyNzkuMzgyQzI1NC4xNTkgMjgwLjAzOSAyNTIuNTMgMjgwLjM3OCAyNTAuODg0IDI4MC4zNzdINS45OTcxOUM0LjgyODcgMjgwLjM3NyAzLjY4NTY4IDI4MC4wMzUgMi43MDg1NSAyNzkuMzkzQzEuNzMxNDMgMjc4Ljc1MSAwLjk2Mjc3MSAyNzcuODM3IDAuNDk3MDIgMjc2Ljc2NEMwLjAzMTI2OTEgMjc1LjY5IC0wLjExMTI4NiAyNzQuNTA0IDAuMDg2ODcxMiAyNzMuMzVDMC4yODUwMjggMjcyLjE5NiAwLjgxNTI2NSAyNzEuMTI2IDEuNjEyNDMgMjcwLjI3TDUzLjMwOTkgMjE0Ljc2OUM1NC40Mjk5IDIxMy41NjYgNTUuNzg0MyAyMTIuNjA3IDU3LjI4OTMgMjExLjk1QzU4Ljc5NDMgMjExLjI5MyA2MC40MTc4IDIxMC45NTMgNjIuMDU5NSAyMTAuOTVIMzA2LjkzM0MzMDguMTAxIDIxMC45NSAzMDkuMjQ0IDIxMS4yOTIgMzEwLjIyMSAyMTEuOTM0QzMxMS4xOTkgMjEyLjU3NiAzMTEuOTY3IDIxMy40OSAzMTIuNDMzIDIxNC41NjRDMzEyLjg5OSAyMTUuNjM3IDMxMy4wNDEgMjE2LjgyNCAzMTIuODQzIDIxNy45NzdDMzEyLjY0NSAyMTkuMTMxIDMxMi4xMTUgMjIwLjIwMSAzMTEuMzE4IDIyMS4wNTdaTTI1OS42NiAxMDkuMjk0QzI1OC41MzcgMTA4LjA4OCAyNTcuMTc4IDEwNy4xMjcgMjU1LjY2OSAxMDYuNDdDMjU0LjE1OSAxMDUuODEzIDI1Mi41MyAxMDUuNDc0IDI1MC44ODQgMTA1LjQ3NUg1Ljk5NzE5QzQuODI4NyAxMDUuNDc1IDMuNjg1NjggMTA1LjgxNyAyLjcwODU1IDEwNi40NTlDMS43MzE0MyAxMDcuMTAxIDAuOTYyNzcxIDEwOC4wMTUgMC40OTcwMiAxMDkuMDg4QzAuMDMxMjY5MSAxMTAuMTYyIC0wLjExMTI4NiAxMTEuMzQ4IDAuMDg2ODcxMiAxMTIuNTAyQzAuMjg1MDI4IDExMy42NTYgMC44MTUyNjUgMTE0LjcyNiAxLjYxMjQzIDExNS41ODJMNTMuMzA5OSAxNzEuMDgzQzU0LjQyOTkgMTcyLjI4NiA1NS43ODQzIDE3My4yNDUgNTcuMjg5MyAxNzMuOTAyQzU4Ljc5NDMgMTc0LjU1OSA2MC40MTc4IDE3NC44OTkgNjIuMDU5NSAxNzQuOTAySDMwNi45MzNDMzA4LjEwMSAxNzQuOTAyIDMwOS4yNDQgMTc0LjU2IDMxMC4yMjEgMTczLjkxOEMzMTEuMTk5IDE3My4yNzYgMzExLjk2NyAxNzIuMzYyIDMxMi40MzMgMTcxLjI4OEMzMTIuODk5IDE3MC4yMTUgMzEzLjA0MSAxNjkuMDI4IDMxMi44NDMgMTY3Ljg3NUMzMTIuNjQ1IDE2Ni43MjEgMzEyLjExNSAxNjUuNjUxIDMxMS4zMTggMTY0Ljc5NUwyNTkuNjYgMTA5LjI5NFpNNS45OTcxOSA2OS40MjY3SDI1MC44ODRDMjUyLjUzIDY5LjQyNzUgMjU0LjE1OSA2OS4wODkgMjU1LjY2OSA2OC40MzJDMjU3LjE3OCA2Ny43NzUxIDI1OC41MzcgNjYuODEzOSAyNTkuNjYgNjUuNjA4MkwzMTEuMzE4IDEwLjEwNjlDMzEyLjExNSA5LjI1MTA3IDMxMi42NDUgOC4xODA1NiAzMTIuODQzIDcuMDI2OTVDMzEzLjA0MSA1Ljg3MzM0IDMxMi44OTkgNC42ODY4NiAzMTIuNDMzIDMuNjEzM0MzMTEuOTY3IDIuNTM5NzQgMzExLjE5OSAxLjYyNTg2IDMxMC4yMjEgMC45ODM5NDFDMzA5LjI0NCAwLjM0MjAyNiAzMDguMTAxIDMuOTUzMTRlLTA1IDMwNi45MzMgMEw2Mi4wNTk1IDBDNjAuNDE3OCAwLjAwMjc5ODY2IDU4Ljc5NDMgMC4zNDMxNCA1Ny4yODkzIDAuOTk5OTUzQzU1Ljc4NDMgMS42NTY3NyA1NC40Mjk5IDIuNjE2MDcgNTMuMzA5OSAzLjgxODQ3TDEuNjI1NzYgNTkuMzE5N0MwLjgyOTM2MSA2MC4xNzQ4IDAuMjk5MzU5IDYxLjI0NCAwLjEwMDc1MiA2Mi4zOTY0Qy0wLjA5Nzg1MzkgNjMuNTQ4OCAwLjA0MzU2OTggNjQuNzM0MiAwLjUwNzY3OSA2NS44MDczQzAuOTcxNzg5IDY2Ljg4MDMgMS43Mzg0MSA2Ny43OTQzIDIuNzEzNTIgNjguNDM3MkMzLjY4ODYzIDY5LjA4MDIgNC44Mjk4NCA2OS40MjQgNS45OTcxOSA2OS40MjY3WiIgZmlsbD0idXJsKCNwYWludDBfbGluZWFyXzQ3Nl8yNDMwKSIvPgo8L2c+CjxkZWZzPgo8bGluZWFyR3JhZGllbnQgaWQ9InBhaW50MF9saW5lYXJfNDc2XzI0MzAiIHgxPSIyNi40MTUiIHkxPSIyODcuMDU5IiB4Mj0iMjgzLjczNSIgeTI9Ii0yLjQ5NTc0IiBncmFkaWVudFVuaXRzPSJ1c2VyU3BhY2VPblVzZSI+CjxzdG9wIG9mZnNldD0iMC4wOCIgc3RvcC1jb2xvcj0iIzk5NDVGRiIvPgo8c3RvcCBvZmZzZXQ9IjAuMyIgc3RvcC1jb2xvcj0iIzg3NTJGMyIvPgo8c3RvcCBvZmZzZXQ9IjAuNSIgc3RvcC1jb2xvcj0iIzU0OTdENSIvPgo8c3RvcCBvZmZzZXQ9IjAuNiIgc3RvcC1jb2xvcj0iIzQzQjRDQSIvPgo8c3RvcCBvZmZzZXQ9IjAuNzIiIHN0b3AtY29sb3I9IiMyOEUwQjkiLz4KPHN0b3Agb2Zmc2V0PSIwLjk3IiBzdG9wLWNvbG9yPSIjMTlGQjlCIi8+CjwvbGluZWFyR3JhZGllbnQ+CjxjbGlwUGF0aCBpZD0iY2xpcDBfNDc2XzI0MzAiPgo8cmVjdCB3aWR0aD0iMzEyLjkzIiBoZWlnaHQ9IjI4MC4zNzciIGZpbGw9IndoaXRlIi8+CjwvY2xpcFBhdGg+CjwvZGVmcz4KPC9zdmc+Cg==&color=9945FF" /></a>
</p>

## Usage

There are 2 ways you can ensure a transaction does not include an advance nonce instruction.


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

Invoke the "never-nonce" program from your program.

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

You can also get a `solana_instruction::Instruction` to perform the cross-program invocation. Add `p-never-nonce` as a dependency with the `"instruction"` feature enabled:

```
cargo add p-never-nonce --features instruction
```

Then use:

```rust
let never_nonce_ix = p_never_nonce::instruction::never_nonce();`
```

## Verification

This build provided on the release uses [`solana-verifiable-build`](https://github.com/solana-foundation/solana-verifiable-build).

* Commit hash: `8598b804be6a10b60a21d78d3b68cca956f19725` (tag `program@v1.0.0`)
* Program ID: `pnn1ctaR1tbP7EGrcz3WtrJKknRxKmKqADztKY9C3YJ`

You can verify it by checking out the repository at the tag `program@v1.0.0` and running:
```
S solana-verify build --library-name pinocchio_never_nonce_program --cargo-build-sbf-args="--tools-version v1.54"
```

> 📝 Note: Currently it is necessary to use unreleased version of solana-verifiable-build ([PR](https://github.com/solana-foundation/solana-verifiable-build/pull/274)) to support passing the extra `--cargo-build-sbf-args` flag).

Once the program is built, it is possible to verify it by running:
```
S solana-verify get-executable-hash target/deploy/pinocchio_never_nonce_program.so
```

The expected hash value is `f9f145339d050163327b345e035a94adfee2b3f68cc8b5d41487403ab0d1f32f`, which matches the on-chain deployed program hash:
```
$ solana-verify get-program-hash -u <MAINNET_RPC_URL> pnn1ctaR1tbP7EGrcz3WtrJKknRxKmKqADztKY9C3YJ
```

## License

The code is licensed under the [Apache License Version 2.0](LICENSE)
