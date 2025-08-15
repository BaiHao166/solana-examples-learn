use std::str::FromStr;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = RpcClient::new_with_commitment(
        String::from("https://devnet.rpcpool.com"),
        CommitmentConfig::confirmed(),
    );

    let wallet = Pubkey::from_str("")?;

    // 添加错误处理
    match client.get_balance(&wallet).await {
        Ok(balance) => {
            println!("Balance: {} SOL", balance as f64 / 1_000_000_000.0);
        },
        Err(e) => {
            eprintln!("获取余额失败: {}", e);
            eprintln!("错误详情:");
            eprintln!("{:?}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}