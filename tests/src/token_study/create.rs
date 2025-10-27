#[cfg(test)]
mod test {
    use crate::DEV_NET_URL;
    use solana_client::nonblocking::rpc_client::RpcClient;
    use solana_sdk::commitment_config::CommitmentConfig;
    use solana_sdk::signature::{Keypair, Signer};
    use solana_sdk::system_instruction::create_account;
    use solana_sdk::transaction::Transaction;
    use spl_token::instruction::initialize_mint;
    use spl_token::solana_program::program_pack::Pack;
    use spl_token::state::Mint;

    #[tokio::test]
    async fn create_token_mint() {
        // 1. 链接开发环境
        let client = RpcClient::new_with_commitment(DEV_NET_URL.to_string(), CommitmentConfig::confirmed());

        // 2. 创建一个付款账户，并请求空投 1SOL
        let fee_payer = Keypair::new();
        let airdrop_signature = client.request_airdrop(&fee_payer.pubkey(), 1_000_000_000).await.unwrap();

        let mut confirmed = client.confirm_transaction(&airdrop_signature).await.unwrap();

        loop {
            if confirmed {
                break;
            }

            confirmed = client.confirm_transaction(&airdrop_signature).await.unwrap();
        }

        // 3. 创建一个Mint Account
        let mint = Keypair::new();
        let space = Mint::LEN;
        let rent = client.get_minimum_balance_for_rent_exemption(space).await.unwrap();
        let create_account_instruction = create_account(
            &fee_payer.pubkey(),
            &mint.pubkey(),
            rent,
            space as u64,
            &spl_token::id(),
        );

        // 4. 初始化Mint Account，即为代币设置基础信息
        let init_mint_instruction = initialize_mint(
            &spl_token::id(), // 调用的合约id
            &mint.pubkey(), // Mint Account 地址
            &fee_payer.pubkey(), // 可以铸造新代币的账户
            Some(&fee_payer.pubkey()), // 可以冻结代币的账户, 可以冻结Token Account
            9, // 代币精度
        ).unwrap();

        // 5. 创建并签署交易
        let hash = client.get_latest_blockhash().await.unwrap();
        let transaction = Transaction::new_signed_with_payer(
            &[create_account_instruction, init_mint_instruction],
            Some(&fee_payer.pubkey()),
            &[&fee_payer, &mint],
            hash,
        );

        // 6. 发送交易
        let signature = client.send_and_confirm_transaction(&transaction).await.unwrap();

        println!("Mint Address: {}", mint.pubkey());
        println!("Transaction Signature: {}", signature);

        let mint_account = client.get_account(&mint.pubkey()).await.unwrap();
        let mint = Mint::unpack(&mint_account.data).unwrap();
        println!("\n{:#?}", mint);
    }
}