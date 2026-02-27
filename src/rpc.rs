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
    V1_1,
    V2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RpcId {
    Number(u128),
    String(String),
}

impl Display for JsonRpcVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            JsonRpcVersion::V1 => "1.0",
            JsonRpcVersion::V1_1 => "1.1",
            JsonRpcVersion::V2 => "2.0",
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcRequest {
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
