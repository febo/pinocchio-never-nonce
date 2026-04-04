#![no_std]

use {
    p_never_nonce::ensure_never_nonce,
    pinocchio::{
        ProgramResult, entrypoint::InstructionContext, error::ProgramError,
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
    // Need at least one account for the instructions sysvar.
    if context.remaining() < 1 {
        return Err(not_enough_account_keys());
    }
    // SAFETY: There is at least one account.
    let instructions_sysvar = unsafe { context.next_account_unchecked().assume_account() };

    ensure_never_nonce(&instructions_sysvar, ProgramError::InvalidArgument)
}

/// Return an error indicating that there are not enough accounts.
///
/// The function is marked as `#[cold]` to hint to the compiler that it is
/// unlikely to be called.
#[cold]
fn not_enough_account_keys() -> ProgramError {
    ProgramError::NotEnoughAccountKeys
}
