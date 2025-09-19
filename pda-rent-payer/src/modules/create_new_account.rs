use crate::modules::rent_vault::RentVault;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint_deprecated::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;

pub fn create_new_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let iter = &mut accounts.iter();
    let new_account = next_account_info(iter)?;
    let rent_vault_account = next_account_info(iter)?;
    let _system_program = next_account_info(iter)?;

    let (rent_vault_pda, rent_vault_bump) = Pubkey::find_program_address(&[RentVault::SEED_PREFIX.as_bytes()], &program_id);
    assert_eq!(rent_vault_account.key, &rent_vault_pda);

    // 创建新账户需要的费用（这里假设是0），我们从租金仓库取这个费用
    let lamports_required_for_rent = Rent::get()?.minimum_balance(0);

    **rent_vault_account.lamports.borrow_mut() -= lamports_required_for_rent;
    **new_account.lamports.borrow_mut() += lamports_required_for_rent;

    Ok(())
}