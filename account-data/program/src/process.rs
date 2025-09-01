use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instructions;
use crate::state::address_info::AddressInfo;

/*
   指令处理函数，智能合约的入口
   客户端调用智能合约时，会将所有的参数发送给这个函数，然后，这个函数根据参数调用不同的业务处理函数
*/
pub fn process_instruction(
    program_id: &Pubkey,      // 当前智能合约地址
    accounts: &[AccountInfo], // 需要的账户信息
    instruction_data: &[u8],  // 客户端调用智能合约时，传入的其他参数值
) -> ProgramResult {
    if let Ok(address_info) = AddressInfo::try_from_slice(instruction_data) {
        return instructions::create::create_address_info(program_id, accounts, address_info);
    }

    Err(ProgramError::InvalidInstructionData)
}
