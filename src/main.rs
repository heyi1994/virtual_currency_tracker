mod models;
mod trader;

use crate::models::app_config::AppConfig;
use crate::models::trader_stream::TraderStream;
use crate::trader::trader_manager::TraderManager;
use anyhow::Result;
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;

#[tokio::main]
async fn main() -> Result<()> {
    let config: AppConfig = toml::from_str(&std::fs::read_to_string("config.toml")?)?;
    let (mut ws_stream, _) = connect_async(config.generate_websocket_url()).await?;
    let mut trader_manager = Box::new(TraderManager::new());

    println!("✅ Connected to Binance trade stream");
    while let Some(msg) = ws_stream.next().await {
        if let Ok(text) = msg?.to_text() {
            if let Ok(currency) = serde_json::from_str::<TraderStream>(text) {
                trader_manager.update(currency)?;
            }
        }
    }

    Ok(())
}
