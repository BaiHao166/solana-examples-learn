pub mod state;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint_deprecated::ProgramResult;
use solana_program::pubkey::Pubkey;

#[cfg(not(feature = "no-entrypoint"))]
use solana_program::entrypoint;
use solana_program::{msg, pubkey, system_instruction, system_program};
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use crate::state::Counter;

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    if system_program::check_id(program_id) {
        msg!("目标合约id与此合约id不一致");
        return Err(ProgramError::IncorrectProgramId);
    }

    let (instruction_discriminant, instruction_data_inner) = instruction_data.split_at(1);

    match instruction_discriminant[0] {
        0 => {
            msg!("指令：自增1");
            return process_increment_counter(accounts, instruction_data_inner);
        },
        1 => {
            msg!("指令：创建计数账户");
            return create_counter(program_id, accounts, instruction_data);
        }
        _ => {
            msg!("Error: 未知指令");
        }
    }

    Ok(())
}

fn create_counter(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let iter = &mut accounts.iter();
    let payer_account = next_account_info(iter)?;
    let counter_account = next_account_info(iter)?;
    let system_program = next_account_info(iter)?;

    // 验证是否是真正的系统程序，防止篡改攻击
    if system_program.key != &system_program::id() {
        msg!("系统程序id校验失败!");
        return Err(ProgramError::IncorrectProgramId);
    }

    if !payer_account.is_signer {
        msg!("计数账户的创建者（即调用此合约的支付者）必须参与签名");
        return Err(ProgramError::MissingRequiredSignature);
    }

    if counter_account.lamports() != 0 {
        msg!("计数账户已存在");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let counter_inst = Counter { count: data[0] as u64 };
    let space = borsh::to_vec(&counter_inst)?.len();
    let lamports_required = Rent::get()?.minimum_balance(space);

    let instruction = system_instruction::create_account(
        payer_account.key,
        counter_account.key,
        lamports_required,
        space as u64,
        program_id
    );

    invoke(
        &instruction,
        &[
            payer_account.clone(),
            counter_account.clone(),
            system_program.clone()
        ]
    )?;

    let counter_data_ref = &mut counter_account.data.borrow_mut();
    let mut counter_data = &mut counter_data_ref[..];

    counter_inst.serialize(&mut counter_data).expect("初始化计数器data失败");

    Ok(())
}

fn process_increment_counter(accounts: &[AccountInfo], _instruction_data: &[u8]) -> ProgramResult {
    let iter = &mut accounts.iter();
    let owner_of_counter_account = next_account_info(iter)?;
    let counter_account = next_account_info(iter)?;

    assert!(owner_of_counter_account.is_signer, "计数账户的拥有者未参与签名！");
    assert!(counter_account.is_writable, "计数账户必须是可写权限!");

    let mut counter = Counter::try_from_slice(&counter_account.try_borrow_data()?)?;
    counter.count += 1;
    counter.serialize(&mut *counter_account.data.borrow_mut())?;

    msg!("计数账户已经自增为：{}", counter.count);

    Ok(())
}