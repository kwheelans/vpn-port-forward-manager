use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use crate::apps::{App, Protocol};

// API Endpoints
const QB_LOGIN_ENDPOINT: &str = "/api/v2/auth/login";
const QB_SET_PREFERENCES_ENDPOINT: &str = "/api/v2/app/setPreferences";

pub struct Qbittorrent {
    pub protocol: Protocol,
    pub hostname: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub port_forward_path: PathBuf,
    pub interval: Duration,
}

impl App for Qbittorrent {
    fn login(&self) {
        todo!()
    }

    fn set_port(&self) {
        todo!()
    }
}


pub fn qb_login_parameters(username: &str, password: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    params.insert("username".into(), username.into());
    params.insert("password".into(), password.into());
    params
}

pub fn qb_login_endpoint(protocol: Protocol, hostname: &str, port: u16) -> String {
    format!(
        "{}://{}:{}{}",
        protocol, hostname, port, QB_LOGIN_ENDPOINT
    )
}

pub fn qb_set_preference_endpoint(protocol: Protocol, hostname: &str, port: u16) -> String {
    format!(
        "{}://{}:{}{}",
        protocol, hostname, port, QB_SET_PREFERENCES_ENDPOINT
    )
}
