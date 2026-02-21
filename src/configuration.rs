use crate::configuration::Protocol::{Http, Https};
use std::collections::HashMap;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tracing::warn;

// API Endpoints
const QB_LOGIN_ENDPOINT: &str = "/api/v2/auth/login";
const QB_SET_PREFERENCES_ENDPOINT: &str = "/api/v2/app/setPreferences";

// Environment Variables
const QB_PROTOCOL: &str = "QB_PROTOCOL";
const QB_HOST: &str = "QB_HOST";
const QB_PORT: &str = "QB_PORT";
const QB_USER: &str = "QB_USER";
const QB_PASSWORD: &str = "QB_PASSWORD";
const PORT_FORWARD_PATH: &str = "PORT_FORWARD_PATH";
const CHECK_INTERVAL: &str = "CHECK_INTERVAL";

// Defaults
const QB_HOST_DEFAULT: &str = "localhost";
const QB_USER_DEFAULT: &str = "admin";
const QB_PASSWORD_DEFAULT: &str = "";
const PORT_FORWARD_PATH_DEFAULT: &str = "/tmp/gluetun/forwarded_port";
const CHECK_INTERVAL_DEFAULT: u64 = 20;

pub enum Protocol {
    Http,
    Https,
}

impl Protocol {
    pub fn default_port(&self) -> u16 {
        match self {
            Http => 80,
            Https => 443,
        }
    }
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Http => "http",
            Https => "https",
        };
        write!(f, "{}", str)
    }
}

pub struct Configuration {
    protocol: Protocol,
    hostname: String,
    port: u16,
    username: String,
    password: String,
    port_forward_path: PathBuf,
    interval: Duration,
}

impl Configuration {
    pub fn new() -> Self {
        let protocol = match std::env::var(QB_PROTOCOL) {
            Ok(value) => {
                if value.eq_ignore_ascii_case("https") {
                    Https
                } else {
                    Http
                }
            }
            _ => Http,
        };
        let port = match std::env::var(QB_PORT) {
            Ok(value) => value.parse::<u16>().unwrap_or_else(|error| {
                warn!("Could not parse: {} -> {}", value, error);
                protocol.default_port()
            }),
            _ => protocol.default_port(),
        };
        let interval = Duration::from_secs(match std::env::var(CHECK_INTERVAL) {
            Ok(value) => value.parse::<u64>().unwrap_or_else(|error| {
                warn!("Could not parse: {} -> {}", value, error);
                CHECK_INTERVAL_DEFAULT
            }),
            _ => CHECK_INTERVAL_DEFAULT,
        });

        Configuration {
            protocol,
            hostname: std::env::var(QB_HOST).unwrap_or(QB_HOST_DEFAULT.into()),
            port,
            username: std::env::var(QB_USER).unwrap_or(QB_USER_DEFAULT.into()),
            password: std::env::var(QB_PASSWORD).unwrap_or(QB_PASSWORD_DEFAULT.into()),
            port_forward_path: std::env::var(PORT_FORWARD_PATH)
                .unwrap_or(PORT_FORWARD_PATH_DEFAULT.into())
                .into(),
            interval,
        }
    }

    pub fn qb_login_parameters(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("username".into(), self.username.clone());
        params.insert("password".into(), self.password.clone());
        params
    }

    pub fn qb_login_endpoint(&self) -> String {
        format!(
            "{}://{}:{}{}",
            self.protocol, self.hostname, self.port, QB_LOGIN_ENDPOINT
        )
    }

    pub fn qb_set_preference_endpoint(&self) -> String {
        format!(
            "{}://{}:{}{}",
            self.protocol, self.hostname, self.port, QB_SET_PREFERENCES_ENDPOINT
        )
    }

    pub fn port_forward_path(&self) -> &Path {
        self.port_forward_path.as_path()
    }

    pub async fn wait(&self) {
        tokio::time::sleep(self.interval).await;
    }
}
