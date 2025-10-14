use serde::{Deserialize, Serialize};
use crate::models::virtual_currency::VirtualCurrency;

#[derive(Deserialize,Serialize)]
pub struct TraderStream {
    ///流事件
    pub stream: String,
    pub data: Option<VirtualCurrency>,
}