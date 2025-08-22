
#[cfg(test)]
mod test {
    use std::str::FromStr;
    use solana_client::nonblocking::rpc_client::RpcClient;
    use solana_sdk::commitment_config::CommitmentConfig;
    use solana_sdk::instruction::{AccountMeta, Instruction};
    use solana_sdk::pubkey::Pubkey;
    use solana_sdk::signature::{Keypair, Signer};
    use solana_sdk::system_program;
    use solana_sdk::transaction::Transaction;
    use crate::{DEV_NET_URL, MY_WALLET_PRIVATE_KEY};

    #[tokio::test]
    async fn test() {
        let program_id = Pubkey::from_str("").unwrap();

        // 1. 连接solana开发网
        let client = RpcClient::new_with_commitment(DEV_NET_URL.to_string(), CommitmentConfig::confirmed());

        // 2. 导入自己的钱包
        let wallet = Keypair::try_from(MY_WALLET_PRIVATE_KEY).expect("导入钱包失败");

        // 3. 创建一个计数器账户
        let counter_keypair = Keypair::new();

        // 3.1 创建计数器的指令
        let instruction = Instruction::new_with_bytes(
            program_id.clone(),
            &[1, 10],
            vec![
                AccountMeta::new(wallet.pubkey(), true),
                AccountMeta::new(counter_keypair.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false)
            ]
        );

        // 3.2 签署交易
        let mut transaction = Transaction::new_with_payer(&[instruction], Some(&wallet.pubkey()));
        let latest_blockhash = client.get_latest_blockhash().await.unwrap();
        transaction.sign(&[&wallet], latest_blockhash);

        // 3.3 发送和确认交易
        let create_signature = client.send_and_confirm_transaction(&transaction).await.unwrap();
        println!("创建计数器成功，签名：{}，公钥：{}", create_signature, counter_keypair.pubkey());

        // 4. 调用指令，使计数器自增

        // 4. 1 生成调用自增方法的指令
        let increment_instruction = Instruction::new_with_bytes(
            program_id,
            &[0],
            vec![
                AccountMeta::new(wallet.pubkey(), true),
                AccountMeta::new(counter_keypair.pubkey(), false),
                AccountMeta::new_readonly(system_program::id(), false)
            ]
        );

        // 4.2 签署交易
        let latest_blockhash = client.get_latest_blockhash().await.unwrap();
        let increment_transaction = Transaction::new_signed_with_payer(
            &[increment_instruction],
            Some(&wallet.pubkey()),
            &[&wallet],
            latest_blockhash
        );

        // 4.3 发送和确认交易
        let signature = client.send_and_confirm_transaction(&increment_transaction).await.unwrap();
        println!("自增成功，签名：{}", signature);
    }
}