use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::pubkey::Pubkey;
use solana_program::{entrypoint, system_instruction};

entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum TransferInstruction {
    CpiTransfer(u64),
    ProgramTransfer(u64),
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = TransferInstruction::try_from_slice(input)?;
    match instruction {
        TransferInstruction::CpiTransfer(args) => transfer_sol_with_cpi(accounts, args),
        TransferInstruction::ProgramTransfer(args) => transfer_sol_with_program(program_id, accounts, args)
    }
}

pub fn transfer_sol_with_cpi(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let iter = &mut accounts.iter();
    let payer = next_account_info(iter)?;
    let recipient = next_account_info(iter)?;
    let system_program = next_account_info(iter)?;

    invoke(
        &system_instruction::transfer(payer.key, recipient.key, amount),
        &[payer.clone(), recipient.clone(), system_program.clone()],
    )?;

    Ok(())
}

pub fn transfer_sol_with_program(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let iter = &mut accounts.iter();
    let payer = next_account_info(iter)?;
    let recipient = next_account_info(iter)?;

    **payer.try_borrow_mut_lamports()? -= amount;
    **recipient.try_borrow_mut_lamports()? += amount;

    Ok(())
}