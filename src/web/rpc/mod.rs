mod send_rpc;

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct RpcInfo {
    pub id: Option<Value>,
    pub method: String,
}
