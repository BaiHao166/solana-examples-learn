use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::{msg, system_program};
use solana_program::program_error::ProgramError;
use crate::modules::state::user::User;
use crate::modules::instructions::{close_user::close_user, create_user::create_user};
#[derive(BorshSerialize, BorshDeserialize)]
pub enum MyInstruction {
    CreateUser(User),
    CloseUser
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8]
) -> ProgramResult {
    if system_program::check_id(program_id) {
        msg!("调用的智能合约id与此智能合约id不一致");
        return Err(ProgramError::IncorrectProgramId);
    }

    let instruction = MyInstruction::try_from_slice(input)?;
    match instruction {
        MyInstruction::CreateUser(user) => create_user(program_id, accounts, user),
        MyInstruction::CloseUser => close_user(accounts)
    }
}