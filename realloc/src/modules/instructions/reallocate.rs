use crate::modules::state::address_info::AddressInfo;
use crate::modules::state::enhanced_address_info::{EnhancedAddressInfo, EnhancedAddressInfoExtender};
use crate::modules::state::work_info::WorkInfo;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::sysvar::Sysvar;

pub fn reallocate_without_zero_init(
    accounts: &[AccountInfo],
    args: EnhancedAddressInfoExtender,
) -> ProgramResult {
    let iter = &mut accounts.iter();
    let target_account = next_account_info(iter)?;
    let payer = next_account_info(iter)?;
    let system_program = next_account_info(iter)?;

    let address_info_data = AddressInfo::try_from_slice(&target_account.data.borrow())?;
    let enhanced_address_info_data = EnhancedAddressInfo::from_address_info(
        address_info_data,
        args.state,
        args.zip,
    );

    let space = borsh::to_vec(&enhanced_address_info_data)?.len();
    let lamport_required = Rent::get()?.minimum_balance(space);

    let diff = lamport_required - target_account.lamports();

    invoke(
        &system_instruction::transfer(payer.key, target_account.key, diff),
        &[
            payer.clone(),
            target_account.clone(),
            system_program.clone(),
        ],
    )?;

    target_account.resize(space)?;

    enhanced_address_info_data.serialize(&mut &mut target_account.data.borrow_mut()[..])?;

    Ok(())
}
pub fn reallocate_zero_init(accounts: &[AccountInfo], data: WorkInfo) -> ProgramResult {
    let iter = &mut accounts.iter();
    let target_account = next_account_info(iter)?;

    let space = borsh::to_vec(&data)?.len();

    target_account.resize(space)?;

    data.serialize(&mut &mut target_account.data.borrow_mut()[..])?;

    Ok(())
}