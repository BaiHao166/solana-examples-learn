use borsh::BorshDeserialize;
use cross_program_invocatio_native_lever::SetPowerStatus;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint;
use solana_program::entrypoint::ProgramResult;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::program::invoke;
use solana_program::pubkey::Pubkey;

entrypoint!(pull_lever);

fn pull_lever(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let iter = &mut accounts.iter();
    let power = next_account_info(iter)?;
    let lever_program = next_account_info(iter)?;

    let power_status = SetPowerStatus::try_from_slice(instruction_data)?;

    let ix = Instruction::new_with_borsh(
        *lever_program.key,
        &power_status,
        vec![AccountMeta::new(*power.key, false)],
    );

    invoke(&ix, &[power.clone()])
}