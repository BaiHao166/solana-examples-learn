
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
    use close_account::modules::state::user::User;
    use close_account::processor::MyInstruction;
    use crate::{DEV_NET_URL, MY_WALLET_PRIVATE_KEY};
    use super::*;

    #[tokio::test]
    async fn test_create_user() {
        // 1. 连接solana开发网
        let client = RpcClient::new_with_commitment(DEV_NET_URL.to_string(), CommitmentConfig::confirmed());

        // 2. 导入自己的钱包
        let wallet_keypair = Keypair::try_from(MY_WALLET_PRIVATE_KEY).expect("导入钱包错误");
        let program_id = Pubkey::from_str("SK6tJai2znL2Qa3c23TPGC59Mu3gHPmcaNsN21r6FFk").expect("设置合约id错误");

        // 3. 创建User实例
        let user = User { name: "张三".to_string() };
        let call_method = MyInstruction::CreateUser(user);
        let user_keypair = Keypair::new();


        // 4. 创建指令
        let instruction = Instruction::new_with_borsh(
            program_id,
            &call_method,
            vec![
                AccountMeta::new(user_keypair.pubkey(), true),
                AccountMeta::new(wallet_keypair.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false)
            ]
        );

        // 5. 创建交易
        let mut transaction = Transaction::new_with_payer(&[instruction], Some(&wallet_keypair.pubkey()));

        // 6. 签署交易
        let latest_blockhash = client.get_latest_blockhash().await.expect("获取最新区块链错误");
        transaction.sign(&[&wallet_keypair, &user_keypair], latest_blockhash);

        // 7. 发送交易
        let sign = client.send_and_confirm_transaction(&transaction).await.expect("交易发送错误");

        // 8GpJzXLyxo2tfYfTc4g2gSBd8CTWraZNfMbea4droEHT
        println!("交易成功: {}", sign);
        println!("用户账户地址: {}", user_keypair.pubkey())
    }

    #[tokio::test]
    async fn test_close_account() {
        // 1. 创建客户端，连接solana开发网
        let client = RpcClient::new_with_commitment(DEV_NET_URL.to_string(), CommitmentConfig::confirmed());

        // 2. 导入自己的钱包
        let wallet_keypair = Keypair::try_from(MY_WALLET_PRIVATE_KEY).expect("导入钱包错误");
        let program_id = Pubkey::from_str("SK6tJai2znL2Qa3c23TPGC59Mu3gHPmcaNsN21r6FFk").expect("设置合约id错误");

        // 3. 设置要关闭的账户公钥
        let closed_account_pubkey = Pubkey::from_str("8GpJzXLyxo2tfYfTc4g2gSBd8CTWraZNfMbea4droEHT").expect("设置关闭账户的公钥错误");

        // 4. 创建指令
        let instruction = Instruction::new_with_borsh(
            program_id,
            &MyInstruction::CloseUser,
            vec![
                AccountMeta::new(closed_account_pubkey, false),
                AccountMeta::new(wallet_keypair.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false)
            ]
        );

        // 5. 创建交易
        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&wallet_keypair.pubkey())
        );

        // 6. 签署交易
        let latest_blockhash = client.get_latest_blockhash().await.expect("获取最新区块链哈希错误");
        transaction.sign(&[&wallet_keypair], latest_blockhash);

        // 7. 发送交易
        let sign = client.send_and_confirm_transaction(&transaction).await.expect("发送交易失败");

        println!("交易成功: {}", sign);

    }


}