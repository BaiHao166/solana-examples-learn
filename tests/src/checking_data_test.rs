
#[cfg(test)]
mod test {
    use std::str::FromStr;
    use solana_client::nonblocking::rpc_client::RpcClient;
    use solana_sdk::commitment_config::CommitmentConfig;
    use solana_sdk::{instruction, system_program};
    use solana_sdk::instruction::{AccountMeta, Instruction};
    use solana_sdk::pubkey::Pubkey;
    use solana_sdk::rent::Rent;
    use solana_sdk::signature::{Keypair, Signer};
    use solana_sdk::transaction::Transaction;
    use solana_sdk::system_instruction::create_account;
    use solana_sdk::sysvar::Sysvar;
    use crate::*;

    #[tokio::test]
    async fn test() {
        test_check().await;
    }

    async fn test_check() {
        let program_id = Pubkey::from_str("33MxDVyzUnZhCC7TCMxwAZx2L3Rx4MxQzTTfR3Y5V1qi").unwrap();
        let wallet_keypair = Keypair::from_bytes(MY_WALLET_PRIVATE_KEY).unwrap();

        // 1. 连接solana测试网
        let rpc_url = String::from(DEV_NET_URL);
        let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

        // 2. 生成需要的账户密钥
        let account_to_create_kaypair = Keypair::new();
        let account_to_change_keypair = Keypair::new();

        // 3. 创建账户
        // let space = 8;
        // let rent = Rent::get().unwrap();
        // let required_lamports = rent.minimum_balance(space);
        //
        let airdrop_signature = client
            .request_airdrop(&account_to_change_keypair.pubkey(), 1_000_000_000).await.unwrap();

        // Wait for airdrop confirmation
        loop {
            if client
                .confirm_transaction(&airdrop_signature).await.unwrap_or(false)
            {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
        }

        // 3. 调用指令
        let instruction = Instruction::new_with_bytes(
            program_id,
            &[],
            vec![
                AccountMeta::new(wallet_keypair.pubkey(), true),
                AccountMeta::new(account_to_create_kaypair.pubkey(), true),
                AccountMeta::new(account_to_change_keypair.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false)
            ]
        );

        // 4. 将指令放入到交易中
        let mut transaction = Transaction::new_with_payer(&[instruction], Some(&wallet_keypair.pubkey()));

        // 5. 签名交易
        let latest_block_hash = client.get_latest_blockhash().await.unwrap();
        transaction.sign(&[&wallet_keypair, &account_to_create_kaypair, &account_to_change_keypair], latest_block_hash);

        // 6. 发送交易
        let sign = client.send_and_confirm_transaction(&transaction).await.unwrap();

        println!("交易成功，交易id为：{}", sign);
        println!("账户地址(创建）为：{}", account_to_create_kaypair.pubkey());
        println!("账户地址(修改)为：{}", account_to_change_keypair.pubkey());
    }
}