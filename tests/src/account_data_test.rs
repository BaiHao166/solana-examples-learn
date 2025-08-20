

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use borsh::BorshDeserialize;
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::commitment_config::CommitmentConfig;
    use solana_sdk::instruction::{AccountMeta, Instruction};
    use solana_sdk::pubkey::Pubkey;
    use solana_sdk::signature::{Keypair, Signer};
    use solana_sdk::system_program;
    use solana_sdk::transaction::Transaction;
    // 从account-data程序中导入需要的模块
    use account_data::{
        state::address_info::AddressInfo
    };
    use crate::MY_WALLET_PRIVATE_KEY;

    #[test]
    fn it_works() {
        let my_wallet = Keypair::from_bytes(MY_WALLET_PRIVATE_KEY).unwrap();

        let program_id = Pubkey::from_str("7iKmQ7nd9FDr3qZGmgZCg1FBPs7ws6iDV8sdDBdH5cRc")
            .expect("智能合约id无效");

        // 1. 连接solana开发网
        let rpc_url = String::from("https://devnet.rpcpool.com");
        let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

        // 2. 创建合约需要的数据
        let payer = my_wallet; // 调用合约和创建数据账户的费用支付者
        let address_info_account = Keypair::new(); // 创建的数据账户的address
        let address_info = AddressInfo::new("张三".to_string(), 1, "中国".to_string(), "上海".to_string());
        let data = borsh::to_vec(&address_info).expect("序列化数据失败");

        println!("数据账户公钥：{}", address_info_account.pubkey());

        // 3. 创建调用合约的指令
        let create_address_instruction = Instruction::new_with_bytes(
            program_id, // 要调用的合约
            &data, // 调用合约时传入的参数
            vec![ // 调用合约时需要的账户信息
                AccountMeta::new(address_info_account.pubkey(), true),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false)
            ]
        );

        // 4. 将指令放在 1个交易中, 需要一个支付者，为本次交易付费
        let mut transaction = Transaction::new_with_payer(&[create_address_instruction], Some(&payer.pubkey()));

        // 5. 签署交易
        let latest_block_hash = client.get_latest_blockhash().expect("获取最新的区块链哈希失败");
        transaction.sign(&[&payer, &address_info_account], latest_block_hash);

        // 6. 发送交易
        let signature = client.send_and_confirm_transaction(&transaction).expect("调用合约失败");

        println!("交易结果：{}", signature)
    }

    #[test]
    fn get_data_from_account() {
        // 5QxETXL6dbJ1yTj4HNbJAVrWu5LPnn7jRdZnVJt5prTK
        let rpc_url = String::from("https://devnet.rpcpool.com");
        let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

        let pubkey = Pubkey::from_str("5QxETXL6dbJ1yTj4HNbJAVrWu5LPnn7jRdZnVJt5prTK").unwrap();
        let account = client.get_account(&pubkey).unwrap();
        println!("{:#?}", account);

        let data = account.data;
        let address_info = AddressInfo::try_from_slice(&data).unwrap();
        println!("{:?}", address_info);
    }
}
