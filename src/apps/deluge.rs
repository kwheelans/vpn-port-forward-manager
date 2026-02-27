use crate::apps::{App, Protocol, endpoint};
use crate::error::Error::{AppResponse, Authorization};
use crate::error::Result;
use crate::rpc::{JsonRpcVersion, RpcId, RpcRequest, RpcResponse};
use reqwest::blocking::{Client};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{debug, warn};

const AUTH_METHOD: &str = "auth.login";
const GET_HOSTS_METHOD: &str = "web.get_hosts";
const CONNECT_METHOD: &str = "web.connect";
const CONNECTED_METHOD: &str = "web.connected";
const SET_CONFIG_METHOD: &str = "core.set_config";
const DELUGE_ENDPOINT: &str = "/json";

#[allow(unused)]
#[derive(Debug)]
struct DelugeHostResponse {
    id: String,
    host: String,
    port: u64,
    username: String,
}

pub struct Deluge {
    pub client: Client,
    pub protocol: Protocol,
    pub hostname: String,
    pub port: u16,
    pub password: String,
    pub port_forward_path: PathBuf,
    pub interval: Duration,
}

impl App for Deluge {
    fn login(&self) -> Result<()> {
        let connected = self.is_connected()?;
        if !connected && self.authorize()? {
            debug!("Deluge {} method success", AUTH_METHOD);
            let hosts = self.get_hosts()?;
                match hosts.first() {
                    None => {
                        warn!("Could not get Deluge Host ID");
                        Err(Authorization)
                    },
                    Some(host) => {
                        debug!("Deluge {} method success", GET_HOSTS_METHOD);
                        self.connect(host.id.as_str())?;
                        Ok(())
                    }
                }
        } else {
            match connected {
                true => {
                    debug!("Deluge is already connected");
                    Ok(())
                },
                false => Err(Authorization),
            }
        }
    }

    fn set_port(&self, port: u16) -> Result<()> {
        let request = RpcRequest::new(
            JsonRpcVersion::V1,
            SET_CONFIG_METHOD,
            json!([{"listen_ports": [port, port]}]),
            generate_id(),
        );
        let response = self.send_rpc_request(&request)?;
        if response.is_success() {
            Ok(())
        } else {
            Err(AppResponse(format!("Deluge {SET_CONFIG_METHOD}")))
        }
    }

    fn interval(&self) -> Duration {
        self.interval
    }

    fn port_forward_path(&self) -> &Path {
        self.port_forward_path.as_path()
    }
}

impl Deluge {
    fn endpoint(&self) -> String {
        endpoint(
            self.protocol,
            self.hostname.as_str(),
            self.port,
            DELUGE_ENDPOINT,
        )
    }

    fn authorize(&self) -> Result<bool> {
        let request = RpcRequest::new(
            JsonRpcVersion::V1,
            AUTH_METHOD,
            Value::Array(vec![Value::String(self.password.clone())]),
            generate_id(),
        );
        let response = self.send_rpc_request(&request)?;
        if response.is_success() && response.result().is_boolean() {
            if response.result().as_bool().unwrap_or_default() {
                Ok(true)
            } else {
                Err(Authorization)
            }
        } else {
            Err(AppResponse(format!("Deluge {AUTH_METHOD}")))
        }
    }

    fn is_connected(&self) -> Result<bool> {
        let request = RpcRequest::new(
            JsonRpcVersion::V1,
            CONNECTED_METHOD,
            Value::Array(vec![]),
            generate_id(),
        );
        let response = self.send_rpc_request(&request)?;
        if response.is_success() && response.result().is_boolean() {
            Ok(response.result().as_bool().unwrap_or_default())
        } else if let Some(error) = response.error() {
            Err(AppResponse(format!(
                "Deluge {CONNECTED_METHOD} with message -> {}",
                error.message()
            )))
        } else {
            Err(AppResponse(format!("Deluge {CONNECTED_METHOD}")))
        }
    }

    fn connect(&self, host_id: &str) -> Result<()> {
        let request = RpcRequest::new(
            JsonRpcVersion::V1,
            CONNECT_METHOD,
            Value::Array(vec![Value::String(host_id.into())]),
            generate_id(),
        );
        let response = self.send_rpc_request(&request)?;
        if !response.is_success() {
            Err(AppResponse(format!(
                "Deluge {CONNECT_METHOD} was unable to connect with host ID {host_id}"
            )))
        } else {
            debug!("Deluge {CONNECT_METHOD} method success");
            Ok(())
        }
    }

    fn get_hosts(&self) -> Result<Vec<DelugeHostResponse>> {
        let request = RpcRequest::new(
            JsonRpcVersion::V1,
            GET_HOSTS_METHOD,
            Value::Array(vec![]),
            generate_id(),
        );
        let response = self.send_rpc_request(&request)?;
        debug!("{GET_HOSTS_METHOD} response: {:?}", response);

        if response.is_success() && response.result().is_array() {
            let mut hosts = Vec::new();
            let response_list: Vec<_> = response
                .result()
                .as_array()
                .unwrap()
                .iter()
                .filter_map(|v| v.as_array())
                .map(|v| v.to_vec())
                .collect();

            for v in response_list {
                debug!("Processing value into DelugeHostResponse: {v:?}");
                let host = DelugeHostResponse {
                    id: v
                        .first()
                        .ok_or_else(|| {
                            AppResponse("Could not get id value for DelugeHostResponse".into())
                        })?
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    host: v
                        .get(1)
                        .ok_or_else(|| {
                            AppResponse("Could not get host value for DelugeHostResponse".into())
                        })?
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    port: v
                        .get(2)
                        .ok_or_else(|| {
                            AppResponse("Could not get port value for DelugeHostResponse".into())
                        })?
                        .as_u64()
                        .unwrap_or_default(),
                    username: v
                        .get(3)
                        .ok_or_else(|| {
                            AppResponse(
                                "Could not get username value for DelugeHostResponse".into(),
                            )
                        })?
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                };
                hosts.push(host)
            }
            debug!("DelugeHostResponses: {:?}", hosts);
            Ok(hosts)
        } else if let Some(error) = response.error() {
            Err(AppResponse(format!(
                "Deluge {GET_HOSTS_METHOD} with message -> {}",
                error.message()
            )))
        } else {
            Err(AppResponse(format!("Deluge {GET_HOSTS_METHOD} ")))
        }
    }

    fn send_rpc_request(&self, request: &RpcRequest) -> Result<RpcResponse> {
        let client = &self.client;
        let response = client.post(self.endpoint()).json(&request).send()?;
        RpcResponse::try_from(response)
    }
}

fn generate_id() -> RpcId {
    RpcId::Number(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime duration_since error")
            .as_nanos(),
    )
}
