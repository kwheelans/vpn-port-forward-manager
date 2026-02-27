mod deluge;
mod qbittorrent;

use crate::LINE_FEED;
use crate::error::Error::{ParsingFailure, PortPath};
use crate::error::Result;
use reqwest::blocking::Client;
use std::fmt::Debug;
use std::path::Path;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use strum::{Display, EnumString};
use tracing::{debug, error, trace, warn};

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
const CHECK_INTERVAL_DEFAULT: u64 = 30;

pub trait App {
    /// Attempts to log in to host and returns error is unsuccessful
    fn login(&self) -> Result<()>;

    /// Attempts to set port value and returns error is unsuccessful
    fn set_port(&self, port: u16) -> Result<()>;
    fn interval(&self) -> Duration;
    fn port_forward_path(&self) -> &Path;
    fn wait(&self) {
        sleep(self.interval());
    }

    fn check_port_forward(&self) -> Result<u16> {
        match self.port_forward_path().try_exists()? {
            true => {
                let value = std::fs::read_to_string(self.port_forward_path())?;
                trace!("Found port value {}", value);
                let value = value.trim_matches(LINE_FEED);
                Ok(value.parse::<u16>()?)
            }
            false => Err(PortPath(format!(
                "Path {} does not exist",
                self.port_forward_path().display()
            ))),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Display, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Application {
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

pub fn app_init() -> Result<Box<dyn App>> {
    let client = Client::builder().cookie_store(true).build()?;
    let application =
        Application::from_str(std::env::var(APPLICATION).unwrap_or_default().as_str())
            .map_err(|_| ParsingFailure(format!("{APPLICATION} value is not valid application type")))?;
    let protocol = Protocol::from_str(std::env::var(PROTOCOL).unwrap_or_default().as_str())
        .unwrap_or_default();
    let port = match std::env::var(PORT) {
        Ok(value) => value.parse::<u16>().unwrap_or_else(|error| {
            warn!("Using default value, could not parse: {} -> {}", value, error);
            application.default_port()
        }),
        _ => application.default_port(),
    };
    let interval = Duration::from_secs(match std::env::var(CHECK_INTERVAL) {
        Ok(value) => value.parse::<u64>().unwrap_or_else(|error| {
            warn!("Using default value, could not parse: {} -> {}", value, error);
            CHECK_INTERVAL_DEFAULT
        }),
        _ => CHECK_INTERVAL_DEFAULT,
    });
    let hostname = std::env::var(HOST).unwrap_or(HOST_DEFAULT.into());
    let username = std::env::var(USER).unwrap_or(USER_DEFAULT.into());
    let password = std::env::var(PASSWORD).unwrap_or(PASSWORD_DEFAULT.into());
    let port_forward_path = std::env::var(PORT_FORWARD_PATH)
        .unwrap_or(PORT_FORWARD_PATH_DEFAULT.into())
        .into();

    // Print selected values
    debug!("application: {}", application);
    debug!("protocol: {}", protocol);
    debug!("port: {}", application);
    debug!("interval: {:?}", interval);
    debug!("hostname: {}", hostname);
    debug!("username: {}", username);
    debug!("port_forward_path: {:?}", port_forward_path);

    Ok(match application {
        Application::QBittorrent => Box::new(qbittorrent::Qbittorrent {
            client,
            protocol,
            port,
            hostname,
            username,
            password,
            port_forward_path,
            interval,
        }),
        Application::Deluge => Box::new(deluge::Deluge {
            client,
            protocol,
            port,
            hostname,
            password,
            port_forward_path,
            interval,
        }),
    })
}

fn endpoint(protocol: Protocol, hostname: &str, port: u16, endpoint: &str) -> String {
    format!("{}://{}:{}{}", protocol, hostname, port, endpoint)
}

pub fn result_to_bool(result: Result<()>) -> bool {
    match result {
        Ok(_) => true,
        Err(error) => {
            error!("{error}");
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::result_to_bool;
    use crate::error::Error::Authorization;
    use crate::error::Result;

    #[test]
    fn result_to_bool_test() {
        let ok: Result<()> = Ok(());
        let error: Result<()> = Err(Authorization);

        assert!(result_to_bool(ok));
        assert!(!result_to_bool(error));
    }
}
