#![no_std]

use pinocchio::{
    AccountView, Address, ProgramResult, error::ProgramError, sysvars::instructions::Instructions,
};

// The system program ID.
const SYSTEM_PROGRAM_ID: Address = Address::new_from_array([0u8; 32]);

/// Ensures that the first top-level instruction in the transaction is not a
/// system program `AdvanceNonceAccount` instruction.
#[inline(always)]
pub fn ensure_never_nonce<E: Into<ProgramError>>(
    instructions_sysvar: &AccountView,
    error: E,
) -> ProgramResult {
    let sysvar = Instructions::try_from(instructions_sysvar)?;

    // Load the first top-level instruction.
    let instruction = sysvar.load_instruction_at(0)?;

    // Reject if the first instruction is a system program
    // `AdvanceNonceAccount` instruction.
    if instruction.get_program_id() == &SYSTEM_PROGRAM_ID
        && let Some([4, 0, 0, 0]) = instruction.get_instruction_data().get(..4)
    {
        Err(error.into())
    } else {
        Ok(())
    }
}
