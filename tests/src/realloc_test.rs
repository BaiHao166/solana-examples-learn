use once_cell::sync::Lazy;
use solana_sdk::signature::Keypair;

static ADDRESS_KEYPAIR: Lazy<Keypair> = Lazy::new(|| {
    Keypair::new()
});

#[cfg(test)]
mod test {
    use crate::realloc_test::ADDRESS_KEYPAIR;
    use crate::{DEV_NET_URL, MY_WALLET_PRIVATE_KEY};
    use realloc::modules::processor::ReallocInstruction;
    use realloc::modules::state::address_info::AddressInfo;
    use realloc::modules::state::enhanced_address_info::EnhancedAddressInfoExtender;
    use solana_client::nonblocking::rpc_client::RpcClient;
    use solana_sdk::commitment_config::CommitmentConfig;
    use solana_sdk::instruction::{AccountMeta, Instruction};
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

        // 3. 创建一个address 数据账户（指令）
        let program_id = Pubkey::try_from("7abRwRJE8V6gzMZHc7MUuABvZ2Tp6Pwu79C4aYVsJuLU").unwrap();
        let address_data = AddressInfo {
            name: "石川澪".to_string(),
            house_number: 108u8,
            street: "Japan Japan".to_string(),
            city: "Japan".to_string(),
        };

        let instruction = Instruction::new_with_borsh(
            program_id,
            &ReallocInstruction::Create(address_data),
            vec![
                AccountMeta::new(ADDRESS_KEYPAIR.pubkey(), true),
                AccountMeta::new(my_wallet.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false)
            ],
        );

        // 4. 创建和签署交易
        let hash = client.get_latest_blockhash().await.unwrap();
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&my_wallet.pubkey()),
            &[&ADDRESS_KEYPAIR, &my_wallet],
            hash,
        );

        // 5. 发送和确认交易
        let signature = client.send_and_confirm_transaction(&transaction).await.unwrap();
        println!("签名：{}", signature)
    }

    #[tokio::test]
    async fn realloc() {
        // 1. 连接solana开发网
        let client = RpcClient::new_with_commitment(DEV_NET_URL.to_string(), CommitmentConfig::confirmed());

        // 2. 导入自己的钱包
        let my_wallet = Keypair::try_from(MY_WALLET_PRIVATE_KEY).unwrap();

        // 3. 构建数据
        let data = EnhancedAddressInfoExtender {
            state: "enable".to_string(),
            zip: 111,
        };

        // 4. 创建指令
        let program_id = Pubkey::try_from("7abRwRJE8V6gzMZHc7MUuABvZ2Tp6Pwu79C4aYVsJuLU").unwrap();
        let instruction = Instruction::new_with_borsh(
            program_id,
            &ReallocInstruction::ReallocateWithoutZeroInit(data),
            vec![
                AccountMeta::new(ADDRESS_KEYPAIR.pubkey(), true),
                AccountMeta::new(my_wallet.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false)
            ],
        );

        // 5. 创建和签署交易
        let hash = client.get_latest_blockhash().await.unwrap();
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&my_wallet.pubkey()),
            &[&ADDRESS_KEYPAIR, &my_wallet],
            hash,
        );

        // 6. 发送和确认交易
        let signature = client.send_and_confirm_transaction(&transaction).await.unwrap();
        println!("realloc签名：{}", signature)
    }
}