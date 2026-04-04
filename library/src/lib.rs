#![no_std]

#[cfg(feature = "instruction")]
extern crate alloc;

#[cfg(feature = "cpi")]
pub mod cpi;
#[cfg(feature = "instruction")]
pub mod instruction;

use pinocchio::{
    AccountView, Address, ProgramResult, error::ProgramError, sysvars::instructions::Instructions,
};

/// The program ID.
///
/// This corresponds to the bytes of the address
/// `pnn1ctaR1tbP7EGrcz3WtrJKknRxKmKqADztKY9C3YJ`.
pub const PROGRAM_ID: Address = Address::new_from_array([
    12, 62, 14, 162, 123, 29, 145, 45, 238, 84, 45, 156, 251, 246, 52, 0, 216, 91, 14, 61, 193,
    148, 45, 109, 11, 9, 54, 154, 31, 32, 228, 247,
]);

// The system program ID.
const SYSTEM_PROGRAM_ID: Address = Address::new_from_array([0u8; 32]);

/// Ensures that the first top-level instruction in the transaction is not a
/// System program `AdvanceNonceAccount` instruction.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_id() {
        let exptected_program_id =
            Address::from_str_const("pnn1ctaR1tbP7EGrcz3WtrJKknRxKmKqADztKY9C3YJ");

        assert_eq!(PROGRAM_ID, exptected_program_id);
    }
}
