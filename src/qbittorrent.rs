use crate::configuration::Protocol;
use std::collections::HashMap;

// API Endpoints
const QB_LOGIN_ENDPOINT: &str = "/api/v2/auth/login";
const QB_SET_PREFERENCES_ENDPOINT: &str = "/api/v2/app/setPreferences";

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