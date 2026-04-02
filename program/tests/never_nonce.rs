#[allow(deprecated)]
use solana_sysvar::recent_blockhashes::{IterItem, RecentBlockhashes};
use {
    bincode::serialize,
    mollusk_svm::{Mollusk, result::Check},
    solana_account::Account,
    solana_hash::Hash,
    solana_instruction::{AccountMeta, Instruction},
    solana_nonce::{
        state::{Data as NonceData, DurableNonce, State as NonceState},
        versions::Versions,
    },
    solana_program_error::ProgramError,
    solana_pubkey::Pubkey,
    solana_system_interface::instruction::{advance_nonce_account, create_nonce_account},
};

// System program ID.
const SYSTEM_PROGRAM_ID: Pubkey = Pubkey::new_from_array([0u8; 32]);

// A dummy program ID for testing nonnonce instructions.
const NEVER_NONCE_ID: Pubkey =
    Pubkey::from_str_const("NeverNonc3333333333333333333333333333333333");

// The size of a nonce account's state.
const NONCE_STATE_SIZE: usize = 80;

/// Create a new Mollusk instance for the given program ID and name.
pub fn mollusk(program_id: &Pubkey, name: &'static str) -> Mollusk {
    unsafe { std::env::set_var("SBF_OUT_DIR", "../target/deploy") };
    solana_logger::setup();

    Mollusk::new(program_id, name)
}

fn system_account_with_lamports(lamports: u64) -> Account {
    Account::new(lamports, 0, &SYSTEM_PROGRAM_ID)
}

fn refresh_nonce(accounts: &mut [(Pubkey, Account)], nonce: Pubkey, authority: Pubkey) {
    let stale_nonce = DurableNonce::from_blockhash(&Hash::new_from_array([9; 32]));
    let stale_state = Versions::new(NonceState::Initialized(NonceData::new(
        authority,
        stale_nonce,
        1,
    )));

    let (_, account) = accounts
        .iter_mut()
        .find(|(key, _)| *key == nonce)
        .expect("nonce account must exist");

    account.data = serialize(&stale_state).expect("nonce state must serialize");
}

fn nononce_instruction() -> Instruction {
    Instruction {
        program_id: NEVER_NONCE_ID,
        accounts: vec![AccountMeta::new_readonly(
            solana_instructions_sysvar::ID,
            false,
        )],
        data: vec![],
    }
}

#[test]
#[allow(deprecated)]
fn success_create_nonce_account() {
    let mut mollusk = mollusk(&NEVER_NONCE_ID, "pinocchio_nononce");
    mollusk.sysvars.recent_blockhashes =
        RecentBlockhashes::from_iter([IterItem(0, &Hash::new_from_array([1; 32]), 1)]);

    let payer = Pubkey::new_unique();
    let nonce = Pubkey::new_unique();
    let authority = payer;

    let recent_blockhashes = mollusk
        .sysvars
        .keyed_account_for_recent_blockhashes_sysvar();
    let rent = mollusk.sysvars.keyed_account_for_rent_sysvar();
    let nonce_rent = mollusk.sysvars.rent.minimum_balance(NONCE_STATE_SIZE);

    let create_nonce = create_nonce_account(&payer, &nonce, &authority, nonce_rent);

    // The test should succeed since we are not advacing the nonce account.

    mollusk.process_and_validate_instruction_chain(
        &[
            // create account
            (&create_nonce[0], &[Check::success()]),
            // initialize nonce
            (&create_nonce[1], &[Check::success()]),
            // nonnonce instruction
            (&nononce_instruction(), &[Check::success()]),
        ],
        &[
            (payer, system_account_with_lamports(nonce_rent * 2)),
            (nonce, Account::default()),
            rent,
            recent_blockhashes,
        ],
    );
}

#[test]
#[allow(deprecated)]
fn reject_advance_nonce_account() {
    let mut mollusk = mollusk(&NEVER_NONCE_ID, "pinocchio_nononce");
    mollusk.sysvars.recent_blockhashes =
        RecentBlockhashes::from_iter([IterItem(0, &Hash::new_from_array([1; 32]), 1)]);

    let payer = Pubkey::new_unique();
    let nonce = Pubkey::new_unique();
    let authority = payer;

    let recent_blockhashes = mollusk
        .sysvars
        .keyed_account_for_recent_blockhashes_sysvar();
    let rent = mollusk.sysvars.keyed_account_for_rent_sysvar();
    let nonce_rent = mollusk.sysvars.rent.minimum_balance(NONCE_STATE_SIZE);

    let create_nonce = create_nonce_account(&payer, &nonce, &authority, nonce_rent);

    let result = mollusk.process_and_validate_instruction_chain(
        &[
            // create account
            (&create_nonce[0], &[Check::success()]),
            // initialize nonce
            (&create_nonce[1], &[Check::success()]),
            // nonnonce instruction
            (&nononce_instruction(), &[Check::success()]),
        ],
        &[
            (payer, system_account_with_lamports(nonce_rent * 2)),
            (nonce, Account::default()),
            rent,
            recent_blockhashes,
        ],
    );

    let mut initialized_accounts = result.resulting_accounts;
    refresh_nonce(&mut initialized_accounts, nonce, authority);

    // Trying to advance a nonce account should fail because we add the
    // "guard" instruction.

    let advance_nonce = advance_nonce_account(&nonce, &authority);

    mollusk.process_and_validate_transaction_instructions(
        &[advance_nonce, nononce_instruction()],
        &initialized_accounts,
        &[Check::err(ProgramError::InvalidArgument)],
    );
}
