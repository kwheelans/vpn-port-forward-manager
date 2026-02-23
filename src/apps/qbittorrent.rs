use crate::apps::{App, Protocol, endpoint};
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tracing::{debug, error, info, warn};

// API Endpoints
const QB_LOGIN_ENDPOINT: &str = "/api/v2/auth/login";
const QB_SET_PREFERENCES_ENDPOINT: &str = "/api/v2/app/setPreferences";

pub struct Qbittorrent {
    pub client: Client,
    pub protocol: Protocol,
    pub hostname: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub port_forward_path: PathBuf,
    pub interval: Duration,
}

impl App for Qbittorrent {
    fn login(&self) -> bool {
        let client = &self.client;
        let response = client
            .post(self.login_endpoint())
            .form(&self.login_parameters())
            .send();

        match response {
            Ok(r) => {
                let status = r.status();
                if status.is_success() {
                    debug!("qBitTorrent login successful");
                    true
                } else {
                    warn!(
                        "qBitTorrent login request failed with status code: {}",
                        status
                    );
                    false
                }
            }
            Err(e) => {
                error!("qBitTorrent login request error: {:?}", e);
                false
            }
        }
    }

    fn set_port(&self, port: u16) -> bool {
        let client = &self.client;
        let json = HashMap::from([("json".to_string(), format!("{{listen_port:{} }}", port))]);
        let response = client
            .post(self.set_preference_endpoint())
            .form(&json)
            .send();

        match response {
            Ok(r) => {
                let status = r.status();
                if status.is_success() {
                    info!("Port updated to {}", port);
                    true
                } else {
                    warn!("Port update request failed with status code: {}", status);
                    false
                }
            }
            Err(e) => {
                error!("Port update request error: {}", e);
                false
            }
        }
    }

    fn interval(&self) -> Duration {
        self.interval
    }

    fn port_forward_path(&self) -> &Path {
        self.port_forward_path.as_path()
    }
}

impl Qbittorrent {
    fn login_parameters(&self) -> HashMap<String, String> {
        HashMap::from([
            ("username".into(), self.username.clone()),
            ("password".into(), self.password.clone()),
        ])
    }

    fn login_endpoint(&self) -> String {
        endpoint(
            self.protocol,
            self.hostname.as_str(),
            self.port,
            QB_LOGIN_ENDPOINT,
        )
    }

    fn set_preference_endpoint(&self) -> String {
        endpoint(
            self.protocol,
            self.hostname.as_str(),
            self.port,
            QB_SET_PREFERENCES_ENDPOINT,
        )
    }
}
