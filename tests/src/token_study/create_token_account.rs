#[cfg(test)]
mod test {
    use crate::DEV_NET_URL;
    use solana_client::nonblocking::rpc_client::RpcClient;
    use solana_sdk::commitment_config::CommitmentConfig;
    use solana_sdk::signature::{Keypair, Signer};
    use solana_sdk::system_instruction::create_account;
    use solana_sdk::transaction::Transaction;
    use spl_token::instruction::{initialize_account, initialize_mint};
    use spl_token::solana_program::program_pack::Pack;
    use spl_token::state::{Account, Mint};

    #[tokio::test]
    async fn test_create_token_account() {
        create_token_account().await.unwrap();
    }

    /**
     * 测试创建 Associated Token Account
     */
    #[tokio::test]
    async fn test_create_ata() {
        create_ata().await.unwrap();
    }

    async fn create_ata() -> anyhow::Result<()> {
        Ok(())
    }

    async fn create_token_account() -> anyhow::Result<()> {
        // 1. 连接solana dev env
        let client = RpcClient::new_with_commitment(DEV_NET_URL.to_string(), CommitmentConfig::confirmed());

        // 2. 创建一个钱包账户
        let fee_payer = Keypair::new();
        let airdrop_signature = client.request_airdrop(&fee_payer.pubkey(), 1_000_000_000).await?;

        client.confirm_transaction(&airdrop_signature).await?;

        // loop {
        //     let confirmed = client.confirm_transaction(&airdrop_signature).await?;
        //
        //     if confirmed {
        //         break;
        //     }
        // }

        // 3. 创建Mint Account
        let mint = Keypair::new();
        let mint_space = Mint::LEN;
        let mint_rent = client.get_minimum_balance_for_rent_exemption(mint_space).await?;
        let create_account_instruction = create_account(
            &fee_payer.pubkey(), // 费用支付者
            &mint.pubkey(), // 要创建的Mint账户地址
            mint_rent,
            mint_space as u64,
            &spl_token::id(), // Mint Account 的 owner是 spl_token_program
        );

        // 初始化Mint Account 中的 data
        let init_mint_instruction = initialize_mint(
            &spl_token::id(), // 要调用的代币程序合约id
            &mint.pubkey(), // 要初始化的Mint Account 地址
            &fee_payer.pubkey(), // 拥有增发代币权限的账户地址
            Some(&fee_payer.pubkey()), // 拥有冻结代币权限的账户地址
            2, // 代币精度
        )?;

        // 4. 创建和签署Mint Account交易
        let hash = client.get_latest_blockhash().await?;
        let transaction = Transaction::new_signed_with_payer(
            &[create_account_instruction, init_mint_instruction],
            Some(&fee_payer.pubkey()),
            &[&fee_payer, &mint],
            hash,
        );

        // 5. 发送并确认交易
        let transaction_signature = client.send_and_confirm_transaction(&transaction).await?;

        let mint_account = client.get_account(&mint.pubkey()).await?;
        let mint_data = Mint::unpack(&mint_account.data)?;

        println!("Mint Address: {}", &mint.pubkey());
        println!("{:#?}", mint_data);
        println!("Transaction Signature: {}", transaction_signature);


        // 6. 创建Token Account
        let token_account = Keypair::new();
        let token_account_space = Account::LEN;
        let token_account_rent = client.get_minimum_balance_for_rent_exemption(token_account_space).await?;
        let create_token_account_instruction = create_account(
            &fee_payer.pubkey(),
            &token_account.pubkey(),
            token_account_rent,
            token_account_space as u64,
            &spl_token::id(),
        );

        // 7. 设置Token Account 的属性值
        let init_token_account_instruction = initialize_account(
            &spl_token::id(), // 要调用的代币程序合约id
            &token_account.pubkey(), // 为哪个 Token Account 设置
            &mint.pubkey(), // Mint Account 的地址
            &fee_payer.pubkey(), // Token Account 的 owner，即钱包地址
        )?;

        // 8. 创建和签署Token Account交易
        let transaction = Transaction::new_signed_with_payer(
            &[create_token_account_instruction, init_token_account_instruction],
            Some(&fee_payer.pubkey()),
            &[&fee_payer, &token_account],
            hash,
        );

        // 9. 发送和确认交易
        let signature = client.send_and_confirm_transaction(&transaction).await?;

        let token = client.get_account(&token_account.pubkey()).await?;
        let token_data = Account::unpack(&token.data)?;

        println!("\nToken Account Address: {}", &token_account.pubkey());
        println!("{:#?}", token_data);
        println!("Transaction Signature: {}", transaction_signature);

        Ok(())
    }
}