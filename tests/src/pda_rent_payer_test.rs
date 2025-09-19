#[cfg(test)]
mod test {
    use crate::{DEV_NET_URL, MY_WALLET_PRIVATE_KEY};
    use pda_rent_payer::modules::init_rent_vault::InitRentVaultArgs;
    use pda_rent_payer::modules::processor::MyInstruction;
    use pda_rent_payer::modules::rent_vault::RentVault;
    use solana_client::nonblocking::rpc_client::RpcClient;
    use solana_sdk::commitment_config::CommitmentConfig;
    use solana_sdk::instruction::{AccountMeta, Instruction};
    use solana_sdk::native_token::LAMPORTS_PER_SOL;
    use solana_sdk::pubkey::Pubkey;
    use solana_sdk::signature::{Keypair, Signer};
    use solana_sdk::system_program;
    use solana_sdk::transaction::Transaction;

    #[tokio::test]
    async fn test() {
        // 1. 连接solana开发网
        let client = RpcClient::new_with_commitment(DEV_NET_URL.to_string(), CommitmentConfig::confirmed());

        // 2. 导入自己的钱包
        let my_wallet = Keypair::try_from(MY_WALLET_PRIVATE_KEY).unwrap();

        // 3. 获得pda地址
        let program_id = Pubkey::try_from("HqKbGCfVKxCi3qzb1SQHHdTTX3EHfmXkEbhQA1VEz4b1").unwrap();
        let (rent_vault_pda, _) = Pubkey::find_program_address(&[RentVault::SEED_PREFIX.as_bytes()], &program_id);

        // 4. 设置租金金库金额
        let init_rent_vault_args = InitRentVaultArgs { fund_lamports: LAMPORTS_PER_SOL };

        // 5. 创建初始化金库指令
        let init_rent_vault_instruction = Instruction::new_with_borsh(
            program_id,
            &MyInstruction::InitRentVault(init_rent_vault_args),
            vec![
                AccountMeta::new(rent_vault_pda, false),
                AccountMeta::new(my_wallet.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false)
            ],
        );

        // 6. 创建新增账户的指令
        let new_account_keypair = Keypair::new();
        let new_account_instruction = Instruction::new_with_borsh(
            program_id,
            &MyInstruction::CreateNewAccount,
            vec![
                AccountMeta::new(new_account_keypair.pubkey(), true),
                AccountMeta::new(rent_vault_pda, false),
                AccountMeta::new_readonly(system_program::id(), false)
            ],
        );

        // 7. 创建并签署交易
        let hash = client.get_latest_blockhash().await.unwrap();
        let transaction = Transaction::new_signed_with_payer(
            &[init_rent_vault_instruction, new_account_instruction],
            Some(&my_wallet.pubkey()),
            &[&my_wallet, &new_account_keypair],
            hash,
        );

        // 8. 发送并确认交易
        let signature = client.send_and_confirm_transaction(&transaction).await.unwrap();
        println!("签名：{}", signature)
    }
}