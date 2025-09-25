use crate::modules::instructions::create::create;
use crate::modules::instructions::reallocate::{reallocate_without_zero_init, reallocate_zero_init};
use crate::modules::state::address_info::AddressInfo;
use crate::modules::state::enhanced_address_info::EnhancedAddressInfoExtender;
use crate::modules::state::work_info::WorkInfo;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum ReallocInstruction {
    Create(AddressInfo),
    ReallocateWithoutZeroInit(EnhancedAddressInfoExtender),
    ReallocateZeroInit(WorkInfo),
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = ReallocInstruction::try_from_slice(input)?;

    match instruction {
        ReallocInstruction::Create(data) => create(program_id, accounts, data)?,
        ReallocInstruction::ReallocateWithoutZeroInit(data) => {
            reallocate_without_zero_init(accounts, data)?
        }
        ReallocInstruction::ReallocateZeroInit(data) => reallocate_zero_init(accounts, data)?
    }

    Ok(())
}

