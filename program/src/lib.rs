#![no_std]

use p_never_nonce::never_nonce;
use pinocchio::{
    ProgramResult,
    entrypoint::{InstructionContext, MaybeAccount},
    error::ProgramError,
    lazy_program_entrypoint, no_allocator, nostd_panic_handler,
};

// Disable the memory allocator.
no_allocator!();
// Use a `no_std` panic handler.
nostd_panic_handler!();
// Process the input lazily.
lazy_program_entrypoint!(process_instruction);

pub fn process_instruction(mut context: InstructionContext) -> ProgramResult {
    let MaybeAccount::Account(instruction_sysvar) = context.next_account()? else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    never_nonce!(instruction_sysvar, ProgramError::InvalidArgument);

    Ok(())
}
