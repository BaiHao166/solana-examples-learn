use once_cell::sync::Lazy;
use solana_sdk::signature::Keypair;

static POWER_KEYPAIR: Lazy<Keypair> = Lazy::new(|| {
    Keypair::new()
});

#[cfg(test)]
mod test {
    use crate::program_invocation_test::POWER_KEYPAIR;
    use crate::{DEV_NET_URL, MY_WALLET_PRIVATE_KEY};
    use cross_program_invocatio_native_lever::{PowerStatus, SetPowerStatus};
    use solana_client::nonblocking::rpc_client::RpcClient;
    use solana_sdk::commitment_config::CommitmentConfig;
    use solana_sdk::instruction::{AccountMeta, Instruction};
    use solana_sdk::pubkey::Pubkey;
    use solana_sdk::signature::{Keypair, Signer};
    use solana_sdk::system_program;
    use solana_sdk::transaction::Transaction;

    #[tokio::test]
    async fn test_init() {


        // 1. 连接solana开发网
        let client = RpcClient::new_with_commitment(DEV_NET_URL.to_string(), CommitmentConfig::confirmed());

        // 2. 导入自己的钱包
        let my_wallet = Keypair::try_from(MY_WALLET_PRIVATE_KEY).unwrap();

        // 3. 创建数据账户需要的数据
        let power_status = PowerStatus { is_on: true };
        let data = borsh::to_vec(&power_status).unwrap();

        // 4. 创建指令
        let program_id = Pubkey::try_from("111").unwrap();
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(POWER_KEYPAIR.pubkey(), true),
                AccountMeta::new(my_wallet.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false)
            ],
        );

        // 5. 签署交易
        let latest_hash = client.get_latest_blockhash().await.unwrap();
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&my_wallet.pubkey()),
            &[&my_wallet, &POWER_KEYPAIR],
            latest_hash,
        );

        // 6. 发送和确认交易
        let signature = client.send_and_confirm_transaction(&transaction).await.unwrap();
        println!("签名：{}", signature);

        test_pull_lever().await;
    }

    // #[tokio::test]
    async fn test_pull_lever() {
        // 1. 连接的solana开发网
        let client = RpcClient::new_with_commitment(DEV_NET_URL.to_string(), CommitmentConfig::confirmed());

        // 2. 导入自己的钱包
        let my_wallet = Keypair::try_from(MY_WALLET_PRIVATE_KEY).unwrap();

        // 3. 创建指令
        // 创建指令数据
        let set_power_status = SetPowerStatus { name: "咚咚熊".to_string() };
        let program_id = Pubkey::try_from("3333").unwrap();
        let level_program_id = Pubkey::try_from("2222").unwrap();
        let instruction = Instruction::new_with_borsh(
            program_id,
            &set_power_status,
            vec![
                AccountMeta::new(POWER_KEYPAIR.pubkey(), false),
                AccountMeta::new_readonly(level_program_id, false),
                AccountMeta::new_readonly(system_program::id(), false)
            ],
        );

        // 4. 签署交易
        let latest_hash = client.get_latest_blockhash().await.unwrap();
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&my_wallet.pubkey()),
            &[&my_wallet],
            latest_hash,
        );

        // 5. 发送和确认交易
        let signature = client.send_and_confirm_transaction(&transaction).await.unwrap();
        println!("pull_lever 签名：{}", signature)
    }
}