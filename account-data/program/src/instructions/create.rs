use borsh::{self, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::sysvar::Sysvar;
use crate::state::address_info::AddressInfo;
pub fn create_address_info(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    address_info: AddressInfo
) -> ProgramResult {
    let account_iter= &mut accounts.iter();
    let address_info_account = next_account_info(account_iter)?;
    let payer = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;

    // 计算创建address 数据账户需要的空间
    let space = borsh::to_vec(&address_info)?.len();
    // 计算创建数据账户需要支付的费用
    let lamports_required = Rent::get()?.minimum_balance(space);

    // 创建一个存储街道地址的数据账户指令
    let instruction = system_instruction::create_account(
        payer.key, // 创建数据账户时谁去支付费用
        address_info_account.key, // 创建的数据账户的address
        lamports_required, // 创建数据账户时需要支付的费用
        space as u64, // 创建的数据账户需要的空间
        program_id // 创建的数据账户的拥有者，这里是这个智能合约
    );

    let array = [
        payer.clone(),
        address_info_account.clone(),
        system_program.clone(),
    ];

    invoke(&instruction, &array)?;

    let address_info_account_rc = &mut address_info_account.data.borrow_mut();
    let mut data = &mut address_info_account_rc[..];
    address_info.serialize(&mut data)?;



    Ok(())
}