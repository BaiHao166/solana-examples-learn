use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::instruction::AccountMeta;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::{msg, system_program};

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8]
) -> ProgramResult {

    // 1. 检查id是否是此智能合约的id
    if system_program::check_id(program_id) {
        return Err(ProgramError::IncorrectProgramId);
    }

    // 2. 检查账户数量是否满足要求
    if accounts.len() < 4 {
        msg!("这个指令需要4个账户: 支付者、被创建的账户、被修改的账户、系统账户");
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let accounts_iter = &mut accounts.iter();
    let _payer = next_account_info(accounts_iter)?;
    let account_create = next_account_info(accounts_iter)?; // 被创建的账户
    let account_change =  next_account_info(accounts_iter)?; // 被修改的账户
    let system_program = next_account_info(accounts_iter)?;

    // 3. 检查账户是否已被创建
    if account_create.lamports() != 0 {
        msg!("账户已经被创建了: {}", account_create.key);
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // 4. 检查要修改的账户的拥有者是否是此智能合约
    if account_change.owner != program_id {
        msg!("需要修改的账户的拥有者不是此智能合约. pubkey: {}  owner: {}", account_change.key, account_change.owner);
        return Err(ProgramError::IllegalOwner);
    }

    // 5. 检查系统账户
    if system_program.key != &system_program::id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    Ok(())
}