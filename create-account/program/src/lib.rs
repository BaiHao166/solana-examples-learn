use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::{entrypoint, msg, system_instruction, system_program};
use solana_program::entrypoint::ProgramResult;
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_program::program::invoke;
use solana_program::pubkey::Pubkey;

entrypoint!(process_instruction);
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8]
) -> ProgramResult {
    let iter = &mut accounts.iter();
    let payer = next_account_info(iter)?;
    let new_account = next_account_info(iter)?;
    let system_program = next_account_info(iter)?;

    msg!("Program invoked. Creating a system account...");
    msg!("  New public key will be: {}", &new_account.key.to_string());

    invoke(
        &system_instruction::create_account(
            payer.key,
            new_account.key,
            LAMPORTS_PER_SOL,
            0,
            &system_program::id()
        ),
        &[
            payer.clone(),
            new_account.clone(),
            system_program.clone()
        ]
    )?;

    msg!("Account created succesfully.");

    Ok(())
}