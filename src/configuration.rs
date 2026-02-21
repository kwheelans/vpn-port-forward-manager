use crate::configuration::Application::{Deluge, QBittorrent};
use crate::qbittorrent::{qb_login_endpoint, qb_login_parameters, qb_set_preference_endpoint};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;
use strum::{Display, EnumString};
use tracing::warn;

// Environment Variables
const APPLICATION: &str = "APPLICATION";
const PROTOCOL: &str = "PROTOCOL";
const HOST: &str = "HOST";
const PORT: &str = "PORT";
const USER: &str = "USER";
const PASSWORD: &str = "PASSWORD";
const PORT_FORWARD_PATH: &str = "PORT_FORWARD_PATH";
const CHECK_INTERVAL: &str = "CHECK_INTERVAL";

// Defaults
const HOST_DEFAULT: &str = "localhost";
const USER_DEFAULT: &str = "admin";
const PASSWORD_DEFAULT: &str = "";
const PORT_FORWARD_PATH_DEFAULT: &str = "/tmp/gluetun/forwarded_port";
const CHECK_INTERVAL_DEFAULT: u64 = 20;

#[derive(Debug, Eq, PartialEq, Default, Clone, Copy, Display, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Application {
    #[default]
    QBittorrent,
    Deluge,
}

#[derive(Debug, Eq, PartialEq, Default, Clone, Copy, Display, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Protocol {
    #[default]
    Http,
    Https,
}

pub struct Configuration {
    application: Application,
    protocol: Protocol,
    hostname: String,
    port: u16,
    username: String,
    password: String,
    port_forward_path: PathBuf,
    interval: Duration,
}

impl Application {
    pub fn default_port(&self) -> u16 {
        match self {
            QBittorrent => 8080,
            Deluge => 8112,
        }
    }
}

impl Configuration {
    pub fn new() -> Self {
        let application =
            Application::from_str(std::env::var(APPLICATION).unwrap_or_default().as_str())
                .unwrap_or_default();
        let protocol = Protocol::from_str(std::env::var(PROTOCOL).unwrap_or_default().as_str())
            .unwrap_or_default();
        let port = match std::env::var(PORT) {
            Ok(value) => value.parse::<u16>().unwrap_or_else(|error| {
                warn!("Could not parse: {} -> {}", value, error);
                application.default_port()
            }),
            _ => application.default_port(),
        };
        let interval = Duration::from_secs(match std::env::var(CHECK_INTERVAL) {
            Ok(value) => value.parse::<u64>().unwrap_or_else(|error| {
                warn!("Could not parse: {} -> {}", value, error);
                CHECK_INTERVAL_DEFAULT
            }),
            _ => CHECK_INTERVAL_DEFAULT,
        });

        Configuration {
            application,
            protocol,
            hostname: std::env::var(HOST).unwrap_or(HOST_DEFAULT.into()),
            port,
            username: std::env::var(USER).unwrap_or(USER_DEFAULT.into()),
            password: std::env::var(PASSWORD).unwrap_or(PASSWORD_DEFAULT.into()),
            port_forward_path: std::env::var(PORT_FORWARD_PATH)
                .unwrap_or(PORT_FORWARD_PATH_DEFAULT.into())
                .into(),
            interval,
        }
    }

    pub fn login_parameters(&self) -> HashMap<String, String> {
        match self.application {
            QBittorrent => qb_login_parameters(self.username.as_str(), self.password.as_str()),
            Deluge => todo!("Deluge login_parameters"),
        }
    }

    pub fn login_endpoint(&self) -> String {
        match self.application {
            QBittorrent => qb_login_endpoint(self.protocol, self.hostname.as_str(), self.port),
            Deluge => todo!("Deluge login_endpoint"),
        }
    }

    pub fn set_preference_endpoint(&self) -> String {
        match self.application {
            QBittorrent => {
                qb_set_preference_endpoint(self.protocol, self.hostname.as_str(), self.port)
            }
            Deluge => todo!("Deluge set_preference_endpoint"),
        }
    }

    pub fn port_forward_path(&self) -> &Path {
        self.port_forward_path.as_path()
    }

    pub async fn wait(&self) {
        tokio::time::sleep(self.interval).await;
    }
}
