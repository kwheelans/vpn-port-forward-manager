use crate::error::Error;
use crate::error::Error::ParsingFailure;
use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};

#[allow(unused)]
#[derive(Debug, Copy, Clone)]
pub enum JsonRpcVersion {
    V1,
    V2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RpcId {
    Number(u128),
    String(String),
}

impl Display for JsonRpcVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            JsonRpcVersion::V1 => "1.0",
            JsonRpcVersion::V2 => "2.0",
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    jsonrpc: Option<String>,
    method: String,
    params: Value,
    id: RpcId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcResponse {
    jsonrpc: Option<String>,
    result: Value,
    error: Option<RpcError>,
    id: Option<RpcId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcError {
    message: String,
    code: i64,
    data: Option<String>,
}

impl RpcRequest {
    pub fn new<S: AsRef<str>>(jsonrpc: JsonRpcVersion, method: S, parms: Value, id: RpcId) -> Self {
        let jsonrpc = match jsonrpc {
            JsonRpcVersion::V1 => None,
            _ => Some(jsonrpc.to_string()),
        };
        Self {
            jsonrpc,
            method: method.as_ref().into(),
            params: parms,
            id,
        }
    }
}

#[allow(unused)]
impl RpcResponse {
    pub fn is_success(&self) -> bool {
        !self.result.is_null() || (self.result.is_null() && self.error.is_none())
    }

    pub fn jsonrpc(&self) -> Option<String> {
        self.jsonrpc.as_ref().map(|v| v.to_owned())
    }

    pub fn result(&self) -> &Value {
        &self.result
    }

    pub fn error(&self) -> Option<RpcError> {
        self.error.as_ref().map(|v| v.to_owned())
    }

    pub fn id(&self) -> Option<RpcId> {
        self.id.as_ref().map(|v| v.to_owned())
    }
}

impl TryFrom<Response> for RpcResponse {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        value
            .json()
            .map_err(|e| ParsingFailure(format!("Could not parse RpcResponse from json -> {e}")))
    }
}

impl RpcError {
    pub fn message(&self) -> &str {
        &self.message
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    const RPC_REQ_V1: &str = r#"{"method":"echo","params":["Hello JSON-RPC"],"id":1}"#;
    const RPC_REQ_V2: &str = r#"{"jsonrpc":"2.0","method":"confirmFruitPurchase","params":[["apple","orange","mangoes"],1.123],"id":"194521489"}"#;
    const RPC_REQ_V2_OBJ: &str =
        r#"{"jsonrpc":"2.0","method":"subtract","params":{"minuend":42,"subtrahend":23},"id":3}"#;

    #[test]
    fn rpc_request_v1() {
        let request = RpcRequest::new(
            JsonRpcVersion::V1,
            "echo",
            json!(["Hello JSON-RPC"]),
            RpcId::Number(1),
        );
        assert_eq!(
            serde_json::to_string(&request).unwrap_or_default(),
            RPC_REQ_V1
        );
    }

    #[test]
    fn rpc_request_v2_params_unnamed() {
        let request = RpcRequest::new(
            JsonRpcVersion::V2,
            "confirmFruitPurchase",
            json!([["apple", "orange", "mangoes"], 1.123]),
            RpcId::String("194521489".into()),
        );
        assert_eq!(
            serde_json::to_string(&request).unwrap_or_default(),
            RPC_REQ_V2
        );
    }

    #[test]
    fn rpc_request_v2_params_named() {
        let request = RpcRequest::new(
            JsonRpcVersion::V2,
            "subtract",
            json!({"minuend": 42, "subtrahend": 23}),
            RpcId::Number(3),
        );
        assert_eq!(
            serde_json::to_string(&request).unwrap_or_default(),
            RPC_REQ_V2_OBJ
        );
    }
}
