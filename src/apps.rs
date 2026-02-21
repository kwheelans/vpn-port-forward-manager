mod qbittorrent;

use std::str::FromStr;
use std::time::Duration;
use strum::{Display, EnumString};
use tracing::warn;
use crate::apps::Application::QBittorrent;

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

pub trait App {
    fn login(&self);
    fn set_port(&self);
    fn init() -> Box<dyn App> where Self: Sized {
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
        let hostname = std::env::var(HOST).unwrap_or(HOST_DEFAULT.into());
        let username = std::env::var(USER).unwrap_or(USER_DEFAULT.into());
        let password = std::env::var(PASSWORD).unwrap_or(PASSWORD_DEFAULT.into());
        let port_forward_path =
            std::env::var(PORT_FORWARD_PATH).unwrap_or(PORT_FORWARD_PATH_DEFAULT.into()).into();
        
        match application {
            Application::QBittorrent => {
                Box::new(qbittorrent::Qbittorrent {
                    protocol,
                    port,
                    hostname,
                    username,
                    password,
                    port_forward_path,
                    interval,
                    
                })
            }
            Application::Deluge => todo!(""),
        }
    }
}

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

impl Application {
    pub fn default_port(&self) -> u16 {
        match self {
            Application::QBittorrent => 8080,
            Application::Deluge => 8112,
        }
    }
}
