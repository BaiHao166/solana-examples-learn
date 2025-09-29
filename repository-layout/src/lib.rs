use crate::processor::process_instruction;
use solana_program::entrypoint;

pub mod modules;
pub mod error;
pub mod processor;

entrypoint!(process_instruction);
