use modules::processor::process_instruction;
use solana_program::entrypoint;

pub mod modules;
pub mod error;

entrypoint!(process_instruction);
