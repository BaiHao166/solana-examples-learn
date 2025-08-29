

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::commitment_config::CommitmentConfig;
    use solana_sdk::instruction::{AccountMeta, Instruction};
    use solana_sdk::pubkey::Pubkey;
    use solana_sdk::signature::{Keypair, Signer};
    use solana_sdk::system_program;
    use solana_sdk::transaction::Transaction;
    use crate::{DEV_NET_URL, MY_WALLET_PRIVATE_KEY};


    #[tokio::test]
    async fn test() {
        // 链接solana开发网
        let client = RpcClient::new_with_commitment(DEV_NET_URL.to_string(), CommitmentConfig::confirmed());

        let wallet_keypair = Keypair::try_from(MY_WALLET_PRIVATE_KEY).unwrap();

        let new_account_keypair = Keypair::new();

        let program_id = Pubkey::from_str("").unwrap();

        // 2. 创建指令
        let instruction = Instruction::new_with_bytes(
            program_id,
            &[],
            vec![
                AccountMeta::new(wallet_keypair.pubkey(), true),
                AccountMeta::new(new_account_keypair.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false)
            ]
        );

        // 3. 创建和签署交易
        let hash = client.get_latest_blockhash().unwrap();
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&wallet_keypair.pubkey()),
            &[&wallet_keypair, &new_account_keypair],
            hash
        );

        // 4. 发送并确认交易
        let sign = client.send_and_confirm_transaction(&transaction).unwrap();

        println!("签名：{}", sign);
    }
}