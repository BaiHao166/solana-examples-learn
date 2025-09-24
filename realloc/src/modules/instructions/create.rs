use crate::modules::state::address_info::AddressInfo;
use borsh::BorshSerialize;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::sysvar::Sysvar;

pub fn create(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: AddressInfo,
) -> ProgramResult {
    let iter = &mut accounts.iter();
    let target_account = next_account_info(iter)?;
    let payer = next_account_info(iter)?;
    let system_program = next_account_info(iter)?;

    let account_space = borsh::to_vec(&data)?.len();
    let rents = Rent::get()?.minimum_balance(account_space);

    invoke(
        &system_instruction::create_account(
            payer.key,
            target_account.key,
            rents,
            account_space as u64,
            program_id,
        ),
        &[
            payer.clone(),
            target_account.clone(),
            system_program.clone()
        ],
    )?;

    data.serialize(&mut &mut target_account.data.borrow_mut()[..])?;
    Ok(())
}