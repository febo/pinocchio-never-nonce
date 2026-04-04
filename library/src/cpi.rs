use {
    crate::PROGRAM_ID,
    pinocchio::{
        AccountView, ProgramResult,
        cpi::{CpiAccount, invoke_signed_unchecked},
        instruction::{InstructionAccount, InstructionView},
    },
};

/// Ensures that the first top-level instruction in the transaction is not a
/// System program `AdvanceNonceAccount` instruction.
pub struct NeverNonce<'account> {
    /// The instructions sysvar account.
    pub instructions_sysvar: &'account AccountView,
}

impl NeverNonce<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        // SAFETY: The instruction only expects read-only accounts.
        unsafe {
            invoke_signed_unchecked(
                &InstructionView {
                    program_id: &PROGRAM_ID,
                    accounts: &[InstructionAccount::readonly(
                        self.instructions_sysvar.address(),
                    )],
                    data: &[],
                },
                &[CpiAccount::from(self.instructions_sysvar)],
                &[],
            );
        }

        Ok(())
    }
}
