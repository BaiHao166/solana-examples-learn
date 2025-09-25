use crate::modules::processor::process_instruction;
use solana_program::entrypoint;

pub mod modules;

entrypoint!(process_instruction);