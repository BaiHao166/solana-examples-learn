use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use solana_program::{msg, system_instruction};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SetPowerStatus {
    pub name: String,
}

/*
    存储电源状态的结构体，该结构体将被存储在数据账户中
 */
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PowerStatus {
    pub is_on: bool,
}

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Ok(power_status) = PowerStatus::try_from_slice(instruction_data) {
        return initialize(program_id, accounts, power_status);
    }

    if let Ok(set_power_status) = SetPowerStatus::try_from_slice(instruction_data) {
        return switch_power(accounts, set_power_status.name);
    }

    Ok(())
}

/*
    初始黄账户：
        1. 创建新的数据账户
        2. 将用户要保存的数据存入到新数据账户中
 */
pub fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    power_status: PowerStatus,
) -> ProgramResult {
    let iter = &mut accounts.iter();
    let power = next_account_info(iter)?; // 要创建的数据账户
    let user = next_account_info(iter)?; // 费用支付者，实际的创建者
    let system_program = next_account_info(iter)?;

    let space = borsh::to_vec(&power_status)?.len();
    let lamports_required = Rent::get()?.minimum_balance(space);

    invoke(
        &system_instruction::create_account(
            user.key,
            power.key,
            lamports_required,
            space as u64,
            program_id, // 由该合约拥有此数据账户
        ),
        &[user.clone(), power.clone(), system_program.clone()],
    )?;

    // 将前端传递的参数保存在数据账户中
    power_status.serialize(&mut &mut power.data.borrow_mut()[..])?;

    Ok(())
}


/*
    开关电源
 */
pub fn switch_power(accounts: &[AccountInfo], name: String) -> ProgramResult {
    let iter = &mut accounts.iter();
    let power = next_account_info(iter)?;

    let mut power_status = PowerStatus::try_from_slice(&power.data.borrow())?;
    power_status.is_on = !power_status.is_on;
    power_status.serialize(&mut &mut power.data.borrow_mut()[..])?;

    msg!("{} is pulling the power switch!", &name);

    match power_status.is_on {
        true => msg!("The power is now on."),
        false => msg!("The power is now off!")
    }

    Ok(())
}