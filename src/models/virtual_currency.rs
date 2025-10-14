use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VirtualCurrency {
    ///最新成交价
    #[serde(rename = "c", deserialize_with = "deserialize_string_to_f64")]
    pub price: f64,
    #[serde(rename = "E")]
    pub time :u64,
    ///时间窗口内价格变动
    #[serde(rename = "p", deserialize_with = "deserialize_string_to_f64")]
    pub price_change: f64,
    ///时间窗口内价格变动百分比
    #[serde(rename = "P", deserialize_with = "deserialize_string_to_f64")]
    pub price_change_percent: f64,
    ///时间窗口内最高价格
    #[serde(rename = "h", deserialize_with = "deserialize_string_to_f64")]
    pub high_price: f64,
    ///时间窗口内最低价格
    #[serde(rename = "l", deserialize_with = "deserialize_string_to_f64")]
    pub low_price: f64,
    ///交易对
    #[serde(rename = "s")]
    pub symbol: String,
}

fn deserialize_string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}
