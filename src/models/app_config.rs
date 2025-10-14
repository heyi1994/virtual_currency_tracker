use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Debug)]
pub struct AppConfig {
    pub window_size: String,
    pub listen_currency: Vec<String>,
}

impl AppConfig {
    ///生成websocket链接
    pub fn generate_websocket_url(&self) -> String {
        let mut url = "wss://stream.binance.com:9443/stream?streams=".to_string();
        for index in 0..self.listen_currency.len() {
            if index > 0 {
                url.push_str("/");
            }
            url.push_str(
                format!(
                    "{}@ticker_{}",
                    &self.listen_currency[index], &self.window_size
                )
                .as_str(),
            );
        }
        url
    }
}
