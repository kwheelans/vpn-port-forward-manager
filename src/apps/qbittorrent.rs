use crate::apps::{App, Protocol, endpoint};
use crate::error::Error::{AppResponse, Authorization, ParsingFailure, PortUpdate};
use crate::error::Result;
use reqwest::blocking::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tracing::{debug, info, trace};

// API Endpoints
const QB_LOGIN_ENDPOINT: &str = "/api/v2/auth/login";
const QB_SET_PREFERENCES_ENDPOINT: &str = "/api/v2/app/setPreferences";
const QB_GET_PREFERENCES_ENDPOINT: &str = "/api/v2/app/preferences";

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
    fn login(&self) -> Result<()> {
        let client = &self.client;
        let request = client
            .post(self.login_endpoint())
            .form(&self.login_parameters())
            .build()?;
        trace!("{:?}", request);
        trace!("{:?}", request.body());

        let response = client.execute(request)?;
        if response.status().is_success() {
            debug!("qBitTorrent login successful");
            Ok(())
        } else {
            debug!(
                "qBitTorrent login request failed with status code: {}",
                response.status()
            );
            Err(Authorization)
        }
    }

    fn set_port(&self, port: u16) -> Result<()> {
        let client = &self.client;
        let json = HashMap::from([("json".to_string(), format!("{{\"listen_port\":{} }}", port))]);
        let response = client
            .post(self.set_preference_endpoint())
            .form(&json)
            .send()?;
        let status = response.status();
        if status.is_success() {
            let actual_port = self.get_current_listen_port()?;
            debug!("actual_port: {:?}", actual_port);
            if port == actual_port {
                info!("Port updated to {}", port);
                Ok(())
            } else {
                Err(PortUpdate(format!(
                    "Actual port {} does not match expected port number {}",
                    actual_port, port
                )))
            }
        } else {
            Err(AppResponse(format!(
                "Port update request failed with status code: {}",
                status
            )))
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

    fn get_preference_endpoint(&self) -> String {
        endpoint(
            self.protocol,
            self.hostname.as_str(),
            self.port,
            QB_GET_PREFERENCES_ENDPOINT,
        )
    }

    fn get_current_listen_port(&self) -> Result<u16> {
        let client = &self.client;
        let response = client.get(self.get_preference_endpoint()).send()?;

        let status = response.status();
        if status.is_success() {
            let json: Value = response.json()?;
            trace!("get preference response json value: {}", json);
            Ok(json
                .as_object()
                .ok_or_else(|| ParsingFailure("unable to parse preferences json object".into()))?
                .get("listen_port")
                .unwrap_or_default()
                .as_number()
                .ok_or_else(|| {
                    ParsingFailure("current listen port json value is not a number".into())
                })?
                .as_u64()
                .unwrap_or_default() as u16)
        } else {
            Err(AppResponse(format!(
                "get preference request failed with status code: {}",
                status
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::MockServer;
    use httpmock::prelude::POST;

    #[test]
    fn login() {
        const USER: &str = "someuser";
        const PASSWORD: &str = "test123456";
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST)
                .header("content-type", "application/x-www-form-urlencoded")
                .path(QB_LOGIN_ENDPOINT)
                .port(server.port())
                .form_urlencoded_tuple("username", USER)
                .form_urlencoded_tuple("password", PASSWORD);
            then.status(200);
        });
        let app_success = Qbittorrent {
            client: Default::default(),
            protocol: Default::default(),
            hostname: server.host(),
            port: server.port(),
            username: USER.to_string(),
            password: PASSWORD.to_string(),
            port_forward_path: Default::default(),
            interval: Default::default(),
        };
        let app_fail = Qbittorrent {
            client: Default::default(),
            protocol: Default::default(),
            hostname: server.host(),
            port: server.port(),
            username: Default::default(),
            password: PASSWORD.to_string(),
            port_forward_path: Default::default(),
            interval: Default::default(),
        };
        let result_success = app_success.login();
        mock.assert();
        assert!(result_success.is_ok());

        let result_fail = app_fail.login();
        assert!(result_fail.is_err());
    }
}
