use crate::modules::rent_vault::RentVault;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke_signed;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::sysvar::Sysvar;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct InitRentVaultArgs {
    pub fund_lamports: u64,
}

/**
 * 初始化租金金库
 */
pub fn init_rent_vault(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: InitRentVaultArgs,
) -> ProgramResult {
    let iter = &mut accounts.iter();
    let rent_vault_account = next_account_info(iter)?;
    let payer = next_account_info(iter)?;
    let system_program = next_account_info(iter)?;

    // 得到pda地址
    let (rent_vault_pda, rent_vault_bump) = Pubkey::find_program_address(&[RentVault::SEED_PREFIX.as_bytes()], program_id);

    assert_eq!(rent_vault_account.key, &rent_vault_pda);

    // 租金金库的应该有多少钱
    let lamports_required = Rent::get()?.minimum_balance(0) + args.fund_lamports;

    // 创建租金金库
    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            &rent_vault_pda,
            lamports_required,
            0,
            program_id,
        ),
        &[payer.clone(), rent_vault_account.clone(), system_program.clone()],
        &[&[RentVault::SEED_PREFIX.as_bytes(), &[rent_vault_bump]]],
    )?;
    Ok(())
}