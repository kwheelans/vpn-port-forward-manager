use crate::apps::{App, Protocol, endpoint};
use crate::rpc::{JsonRpcVersion, RpcId, RpcRequest, RpcResponse};
use reqwest::blocking::{Client, Response};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{debug, error, warn};

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
    fn login(&self) -> bool {
        let mut successful = self.is_connected();
        if !successful && self.authorize() {
            debug!("Deluge {} method success", AUTH_METHOD);
            if let Some(hosts) = self.get_hosts() {
                match hosts.first() {
                    None => warn!("Deluge {} method failed", GET_HOSTS_METHOD),
                    Some(host) => {
                        debug!("Deluge {} method success", GET_HOSTS_METHOD);
                        match self.connect(host.id.as_str()) {
                            true => {
                                debug!("Deluge {} method success", CONNECT_METHOD);
                                successful = true;
                            }
                            false => {
                                warn!("Deluge {} method failed", CONNECT_METHOD);
                            }
                        }
                    }
                }
            }
        } else {
            match successful {
                true => debug!("Deluge is already connected"),
                false => warn!("Deluge {} method failed", AUTH_METHOD),
            }
        }
        successful
    }

    fn set_port(&self, port: u16) -> bool {
        let request = RpcRequest::new(
            JsonRpcVersion::V1,
            SET_CONFIG_METHOD,
            json!([{"listen_ports": [port, port]}]),
            generate_id(),
        );
        debug!("{:?}", request);
        let response = self.send_rpc_request(&request);
        match handle_rpc_response(response) {
            None => false,
            Some(rpc) => rpc.is_success(),
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

    fn authorize(&self) -> bool {
        let request = RpcRequest::new(
            JsonRpcVersion::V1,
            AUTH_METHOD,
            Value::Array(vec![Value::String(self.password.clone())]),
            generate_id(),
        );
        let response = self.send_rpc_request(&request);
        match handle_rpc_response(response) {
            None => false,
            Some(rpc) => {
                if rpc.is_success() && rpc.result().is_boolean() {
                    rpc.result().as_bool().unwrap()
                } else {
                    false
                }
            }
        }
    }

    fn is_connected(&self) -> bool {
        let request = RpcRequest::new(
            JsonRpcVersion::V1,
            CONNECTED_METHOD,
            Value::Array(vec![]),
            generate_id(),
        );
        let response = self.send_rpc_request(&request);
        match handle_rpc_response(response) {
            None => false,
            Some(rpc) => {
                if rpc.is_success() && rpc.result().is_boolean() {
                    rpc.result().as_bool().unwrap_or_default()
                } else {
                    false
                }
            }
        }
    }

    fn connect(&self, hostid: &str) -> bool {
        let request = RpcRequest::new(
            JsonRpcVersion::V1,
            CONNECT_METHOD,
            Value::Array(vec![Value::String(hostid.into())]),
            generate_id(),
        );
        let response = self.send_rpc_request(&request);
        match handle_rpc_response(response) {
            None => false,
            Some(rpc) => rpc.is_success(),
        }
    }

    fn get_hosts(&self) -> Option<Vec<DelugeHostResponse>> {
        let request = RpcRequest::new(
            JsonRpcVersion::V1,
            GET_HOSTS_METHOD,
            Value::Array(vec![]),
            generate_id(),
        );
        let response = self.send_rpc_request(&request);
        match handle_rpc_response(response) {
            None => None,
            Some(rpc) => {
                debug!("{:?}", rpc);

                if rpc.is_success() && rpc.result().is_array() {
                    let response: Vec<_> = rpc
                        .result()
                        .as_array()
                        .unwrap()
                        .iter()
                        .filter_map(|v| v.as_array())
                        .map(|v| v.to_vec())
                        .collect();
                    let mut hosts = Vec::new();
                    for v in response {
                        let host = DelugeHostResponse {
                            id: v.first()?.as_str().unwrap_or_default().to_string(),
                            host: v.get(1)?.as_str().unwrap_or_default().to_string(),
                            port: v.get(2)?.as_u64().unwrap_or_default(),
                            username: v.get(3)?.as_str().unwrap_or_default().to_string(),
                        };
                        hosts.push(host)
                    }
                    debug!("DelugeHostResponses: {:?}", hosts);
                    Some(hosts)
                } else {
                    None
                }
            }
        }
    }

    fn send_rpc_request(&self, request: &RpcRequest) -> reqwest::Result<Response> {
        let client = &self.client;
        client.post(self.endpoint()).json(&request).send()
    }
}

fn handle_rpc_response(result: reqwest::Result<Response>) -> Option<RpcResponse> {
    match result {
        Ok(response) => match RpcResponse::try_from(response) {
            Ok(rpc) => {
                debug!("{:?}", rpc);
                Some(rpc)
            }
            Err(e) => {
                error!("RpcResponse deserialize failed -> {}", e);
                None
            }
        },
        Err(e) => {
            error!("Response error -> {}", e);
            None
        }
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
