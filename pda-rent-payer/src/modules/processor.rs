use crate::modules::create_new_account::create_new_account;
use crate::modules::init_rent_vault::{init_rent_vault, InitRentVaultArgs};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint_deprecated::ProgramResult;
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum MyInstruction {
    InitRentVault(InitRentVaultArgs),
    CreateNewAccount,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = MyInstruction::try_from_slice(input)?;

    match instruction {
        MyInstruction::InitRentVault(initRentVaultArgs) => init_rent_vault(program_id, accounts, initRentVaultArgs),
        MyInstruction::CreateNewAccount => create_new_account(program_id, accounts)
    }
}