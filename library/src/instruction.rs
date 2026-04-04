use {
    crate::PROGRAM_ID,
    alloc::vec,
    pinocchio::sysvars::instructions::INSTRUCTIONS_ID,
    solana_instruction::{AccountMeta, Instruction},
};

/// Return an instruction that can be used to ensure that the
/// first top-level instruction in the transaction is not a
/// System program `AdvanceNonceAccount` instruction.
#[inline]
pub fn never_nonce() -> Instruction {
    Instruction::new_with_bytes(
        PROGRAM_ID,
        &[],
        vec![AccountMeta::new_readonly(INSTRUCTIONS_ID, false)],
    )
}
