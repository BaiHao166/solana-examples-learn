use crate::modules::state::Favorites;
use borsh::BorshSerialize;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use solana_program::{msg, system_instruction};

pub fn create_pda(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: Favorites,
) -> ProgramResult {
    let iter = &mut accounts.iter();
    let user = next_account_info(iter)?; // 签署交易的用户
    let favorite_account = next_account_info(iter)?; // 这个账户将被创建
    let system_program = next_account_info(iter)?;

    // 1. 生成PDA
    let (favorite_pda, favorite_bump) = Pubkey::find_program_address(&[b"favorite", user.key.as_ref()], program_id);

    // 2. 检查要创建的收藏账户是否是按照规则生成的PDA地址
    if favorite_account.key != &favorite_pda {
        return Err(ProgramError::IncorrectProgramId);
    }

    // 3. 检查要创建的收藏账户是否已经存在
    if favorite_account.data.borrow().len() == 0 {

        // 4. 创建pda地址的收藏账户
        let space = borsh::to_vec(&data)?.len();
        let lamports = Rent::get()?.minimum_balance(space);

        let instruction = system_instruction::create_account(
            user.key,
            favorite_account.key,
            lamports,
            space as u64,
            program_id,
        );

        invoke_signed(
            &instruction,
            &[user.clone(), favorite_account.clone(), system_program.clone()],
            &[&[b"favorite", user.key.as_ref(), &[favorite_bump]]],
        )?;

        data.serialize(&mut &mut favorite_account.data.borrow_mut()[..])?;
        msg!("{:#?}", data);
    } else {
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    Ok(())
}