//! Ensure a transaction does not include an advance nonce instruction.
//!
//! There are 2 ways you can ensure a transaction does not include an advance
//! nonce instruction.
//!
//! # 1) Library
//!
//! Add `p-never-nonce` as a dependency:
//!
//! ```text
//! cargo add p-never-nonce
//! ```
//!
//! In your program add a call to `ensure_never_nonce`.
//!
//! ## Example
//!
//! ```
//! use {
//!     p_never_nonce::ensure_never_nonce,
//!     pinocchio::{
//!         ProgramResult, entrypoint::{InstructionContext, MaybeAccount}, error::ProgramError,
//!         lazy_program_entrypoint, no_allocator, nostd_panic_handler,
//!     },
//! };
//!
//! // Disable the memory allocator.
//! no_allocator!();
//! // Use a `no_std` panic handler.
//! nostd_panic_handler!();
//! // Process the input lazily.
//! lazy_program_entrypoint!(process_instruction);
//!
//! pub fn process_instruction(mut context: InstructionContext) -> ProgramResult {
//!     let MaybeAccount::Account(instructions_sysvar) = context.next_account()? else {
//!         return Err(ProgramError::NotEnoughAccountKeys);
//!     };
//!     // Fails if the first instruction is an advance nonce.
//!     ensure_never_nonce(&instructions_sysvar, ProgramError::InvalidArgument)
//! }
//! ```
//!
//! # 2) Cross-program invocation
//!
//! Add `p-never-nonce` as a dependency with the `"cpi"` feature enabled:
//!
//! ```text
//! cargo add p-never-nonce --features cpi
//! ```
//!
//! Invoke the "never-nonce" program from your program.
//!
//! ## Example
//!
//! ```
//! use {
//!     p_never_nonce::cpi::NeverNonce,
//!     pinocchio::{
//!         ProgramResult, entrypoint::{InstructionContext, MaybeAccount}, error::ProgramError,
//!         lazy_program_entrypoint, no_allocator, nostd_panic_handler,
//!     },
//! };
//!
//! // Disable the memory allocator.
//! no_allocator!();
//! // Use a `no_std` panic handler.
//! nostd_panic_handler!();
//! // Process the input lazily.
//! lazy_program_entrypoint!(process_instruction);
//!
//! pub fn process_instruction(mut context: InstructionContext) -> ProgramResult {
//!     let MaybeAccount::Account(ref instructions_sysvar) = context.next_account()? else {
//!         return Err(ProgramError::NotEnoughAccountKeys);
//!     };
//!     // The "never-nonce" CPI will return an error if the first instruction
//!     // is an advance nonce.
//!     NeverNonce {
//!         instructions_sysvar,
//!     }
//!     .invoke()
//! }
//! ```
//!
//! You can also get a `solana_instruction::Instruction` to perform the
//! cross-program invocation. Add `p-never-nonce` as a dependency with the
//! `"instruction"` feature enabled:
//!
//! ```text
//! cargo add p-never-nonce --features instruction
//! ```
//!
//! Then use:
//!
//! ```no_run
//! let never_nonce_ix = p_never_nonce::instruction::never_nonce();
//! ```

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

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
