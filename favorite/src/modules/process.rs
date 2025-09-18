use crate::modules::instructions::create_pda::create_pda;
use crate::modules::instructions::get_pda::get_pda;
use crate::modules::state::Favorites;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint_deprecated::ProgramResult;
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum FavoritesInstruction {
    CreatePda(Favorites),
    GetPad,
}

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = FavoritesInstruction::try_from_slice(instruction_data)?;

    match instruction {
        FavoritesInstruction::CreatePda(favorites) => create_pda(program_id, accounts, favorites),
        FavoritesInstruction::GetPad => get_pda(program_id, accounts)
    }?;

    Ok(())
}