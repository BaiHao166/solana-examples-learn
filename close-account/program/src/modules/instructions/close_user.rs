use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;

pub fn close_user(accounts: &[AccountInfo]) -> ProgramResult {
    if accounts.len() < 3 {
        msg!("缺少必要账户");
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let accounts_iter = &mut accounts.iter();
    let target_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    if target_account.lamports() == 0 {
        msg!("账户已被关闭或不存在");
        return Err(ProgramError::UninitializedAccount);
    }

    // 计算 0 空间所需的最小费用
    let account_space: usize = 0;
    let lamports_required = Rent::get()?.minimum_balance(account_space);

    // 计算被关闭账户剩余的余额
    let diff = **target_account.lamports.borrow_mut() - lamports_required;

    // 被关闭账户只保留关闭账户操作需要的花费
    **target_account.lamports.borrow_mut() -= diff;

    // 被关闭账户剩余的费用退回给用户
    **payer.lamports.borrow_mut() += diff;

    // 将账户数据空间重置为0
    target_account.resize(account_space)?;
    // 将账户的拥有者设置为系统账户，相当于任何人不再拥有此账户
    target_account.assign(system_program.key);

    Ok(())
}