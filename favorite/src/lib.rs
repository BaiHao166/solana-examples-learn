use crate::modules::process::process;
use solana_program::entrypoint;

pub mod modules;

entrypoint!(process);