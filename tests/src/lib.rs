use solana_sdk::signature::Keypair;
use std::io::Read;

mod account_data_test;
mod checking_data_test;
mod close_account_test;
mod counter_test;
mod create_account_test;
mod program_invocation_test;
mod favorite_test;

const DEV_NET_URL: &str ="https://devnet.rpcpool.com";
const MY_WALLET_PUBKEY: &str = "3UVeQEnPyjsZb87kZd6mueKJVwQYoKTFoKrC2sD8RY4m";
const MY_WALLET_PRIVATE_KEY: &[u8] = &[57,31,147,125,165,139,15,249,52,197,17,228,255,110,178,89,214,19,139,250,164,200,21,5,199,57,38,39,228,186,181,199,36,194,113,204,19,57,114,146,117,28,181,50,5,220,54,239,194,26,86,82,105,107,121,30,71,108,236,42,74,153,10,230];