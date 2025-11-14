use claude_confirm::mcp::run_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日誌
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();

    log::info!("啟動 Claude Confirm MCP 服務器");

    run_server().await?;

    Ok(())
}
