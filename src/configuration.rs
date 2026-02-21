use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

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

pub struct Configuration {
    pub application: Application,
    protocol: Protocol,
    hostname: String,
    port: u16,
    username: String,
    password: String,
    port_forward_path: PathBuf,
    interval: Duration,
}


impl Configuration {
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
