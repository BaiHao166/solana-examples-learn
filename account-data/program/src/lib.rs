pub mod state;
pub mod instructions;
pub mod process;

use solana_program::entrypoint;

use process::process_instruction;

// 智能合约的入口，客户端调用智能合约时，就是在调用这里指定的函数
entrypoint!(process_instruction);