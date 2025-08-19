use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::{msg, system_instruction};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use crate::modules::state::user::User;
use borsh::{BorshSerialize, to_vec};
use solana_program::program::invoke_signed;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;

pub fn create_user(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: User
) -> ProgramResult {

    if accounts.len() < 3 {
        msg!("账户数量错误");
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    // 1. 获取需要使用的账户
    let accounts_iter = &mut accounts.iter();
    // 需要被创建的账户
    let target_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    if target_account.lamports() != 0 {
        msg!("账户已经存在了 {}", target_account.key);
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // 2. 设置创建账户需要的信息
    // 2.1 计算创建账户需要的费用
    let account_space = to_vec(&data)?.len(); // data 将被放在账户的data属性中
    let lamports_required  = Rent::get()?.minimum_balance(account_space);

    // 2.2 获取PDA地址: 固定前缀 + 账户创建费用支付者的地址（即需要创建此账户的用户） + 此合约id
    //
    let (_, bump) = Pubkey::find_program_address(
        &[User::SEED_PREFIX.as_bytes(), payer.key.as_ref()],
        program_id
    );

    // 3. 创建账户
    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            target_account.key,
            lamports_required,
            account_space as u64,
            program_id,
        ),
        &[
            payer.clone(),
            target_account.clone(),
            system_program.clone()
        ],
        // 由于PDA没有密钥，所以我们用生成PAD的 seed 来签名
        &[&[User::SEED_PREFIX.as_bytes(), payer.key.as_ref(), &[bump]]],
    )?;

    // 4. 将数据写入账户的data属性中
    data.serialize(&mut &mut target_account.data.borrow_mut()[..])?;

    Ok(())
}