#[cfg(test)]
mod test {
    use crate::{DEV_NET_URL, MY_WALLET_PRIVATE_KEY};
    use favorite::modules::process::FavoritesInstruction;
    use favorite::modules::state::Favorites;
    use solana_client::nonblocking::rpc_client::RpcClient;
    use solana_sdk::commitment_config::CommitmentConfig;
    use solana_sdk::instruction::{AccountMeta, Instruction};
    use solana_sdk::pubkey::Pubkey;
    use solana_sdk::signature::{Keypair, Signer};
    use solana_sdk::system_program;
    use solana_sdk::transaction::Transaction;

    #[tokio::test]
    async fn test_create_pda() {
        // 1. 连接solana 开发环境
        let client = RpcClient::new_with_commitment(DEV_NET_URL.to_string(), CommitmentConfig::confirmed());

        // 2. 导入自己的钱包
        let my_wallet = Keypair::try_from(MY_WALLET_PRIVATE_KEY).unwrap();

        // 3. 创建数据
        let favorites = Favorites {
            number: 001,
            color: "super blue".to_string(),
            hobbies: vec!["swimming".to_string(), "running".to_string()],
        };

        // 4. 创建pda 地址
        let program_id = Pubkey::try_from("7isda2oEFG5Z4K3LCLA8YKmTyTDcqgvKbtYn6kH75KUW").unwrap();
        let (favorite_pda, favorite_bump) = Pubkey::find_program_address(&[b"favorite", my_wallet.pubkey().as_ref()], &program_id);

        // 5. 创建指令
        let instruction = Instruction::new_with_borsh(
            program_id,
            &FavoritesInstruction::CreatePda(favorites),
            vec![
                AccountMeta::new(my_wallet.pubkey(), true),
                AccountMeta::new(favorite_pda, false),
                AccountMeta::new_readonly(system_program::id(), false)
            ],
        );

        // 6. 签署交易
        let hash = client.get_latest_blockhash().await.unwrap();
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&my_wallet.pubkey()),
            &[&my_wallet],
            hash,
        );

        // 7. 发送并确认交易
        let signature = client.send_and_confirm_transaction(&transaction).await.unwrap();
        println!("signature: {}", signature)
    }

    #[tokio::test]
    async fn test_get_pda() {
        // 1. 连接solana 开发环境
        let client = RpcClient::new_with_commitment(DEV_NET_URL.to_string(), CommitmentConfig::confirmed());
        // 2. 导入自己的钱包
        let my_wallet = Keypair::try_from(MY_WALLET_PRIVATE_KEY).unwrap();

        // 3. 得到要查询的pda地址
        let program_id = Pubkey::try_from("7isda2oEFG5Z4K3LCLA8YKmTyTDcqgvKbtYn6kH75KUW").unwrap();
        let (favorite_pda, _) = Pubkey::find_program_address(&[b"favorite", my_wallet.pubkey().as_ref()], &program_id);

        // 4. 创建指令
        let instruction = Instruction::new_with_borsh(
            program_id,
            &FavoritesInstruction::GetPad,
            vec![
                AccountMeta::new(my_wallet.pubkey(), true),
                AccountMeta::new(favorite_pda, false),
                AccountMeta::new_readonly(system_program::id(), false)
            ],
        );

        // 5. 创建交易
        let hash = client.get_latest_blockhash().await.unwrap();
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&my_wallet.pubkey()),
            &[&my_wallet],
            hash,
        );

        // 6. 发送并确认交易
        let signature = client.send_and_confirm_transaction(&transaction).await.unwrap();
        println!("signature: {}", signature)
    }
}