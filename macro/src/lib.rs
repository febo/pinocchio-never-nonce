#![no_std]

#[macro_export]
macro_rules! never_nonce {
    ( $sysvar_account:expr, $error:expr ) => {
        const SYSTEM_PROGRAM_ID: ::pinocchio::Address =
            ::pinocchio::Address::new_from_array([0u8; 32]);

        let sysvar = ::pinocchio::sysvars::instructions::Instructions::try_from(&$sysvar_account)?;
        // Load the first top-level instruction.
        let instruction = sysvar.load_instruction_at(0)?;
        // Reject the transaction if the first instruction  is a system program
        // `AdvanceNonceAccount` instruction.
        if instruction.get_program_id() == &SYSTEM_PROGRAM_ID
            && let Some([4, 0, 0, 0]) = instruction.get_instruction_data().get(..4)
        {
            return Err($error);
        }
    };
}
