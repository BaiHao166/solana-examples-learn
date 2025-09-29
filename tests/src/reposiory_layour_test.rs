#[cfg(test)]
mod tests {
    use crate::{DEV_NET_URL, MY_WALLET_PRIVATE_KEY};
    use repository_layout::modules::processor::CarnivalInstructionData;
    use solana_client::nonblocking::rpc_client::RpcClient;
    use solana_sdk::commitment_config::CommitmentConfig;
    use solana_sdk::instruction::{AccountMeta, Instruction};
    use solana_sdk::pubkey::Pubkey;
    use solana_sdk::signature::{Keypair, Signer};
    use solana_sdk::transaction::Transaction;

    #[tokio::test]
    async fn test() {
        // 1. 连接solana 开发网
        let client = RpcClient::new_with_commitment(DEV_NET_URL.to_string(), CommitmentConfig::confirmed());
        // 2. 导入自己的钱包
        let my_wallet = Keypair::try_from(MY_WALLET_PRIVATE_KEY).unwrap();

        // 3. 创建指令
        let program_id = Pubkey::try_from("7abRwRJE8V6gzMZHc7MUuABvZ2Tp6Pwu79C4aYVsJuLU").unwrap();

        let data = CarnivalInstructionData {
            name: "正在演戏的河北彩花".to_string(),
            height: 180,
            ticket_count: 10,
            attraction: "game".to_string(),
            attraction_name: "三角洲行动".to_string(),
        };

        let instruction = Instruction::new_with_borsh(
            program_id,
            &data,
            vec![
                AccountMeta::new(my_wallet.pubkey(), true)
            ],
        );

        // 4. 创建和签署交易
        let hash = client.get_latest_blockhash().await.unwrap();
        let mut transaction = Transaction::new_with_payer(&[instruction], Some(&my_wallet.pubkey()));
        transaction.sign(&[my_wallet], hash);

        // 5. 发送和确认交易
        let signature = client.send_and_confirm_transaction(&transaction).await.unwrap();
        println!("signature {}", signature);
    }
}