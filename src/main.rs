use crate::configuration::Configuration;
use reqwest::Client;
use std::collections::HashMap;
use std::str::FromStr;
use tracing::{debug, error, info, warn};
use tracing_subscriber::filter::LevelFilter;

mod configuration;

const LINE_FEED: char = '\n';
const LOG_LEVEL: &str = "LOG_LEVEL";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(log_level()).init();
    let client = Client::builder().cookie_store(true).build()?;
    let config = Configuration::new();
    run(&client, &config).await;
    Ok(())
}

async fn run(client: &Client, config: &Configuration) {
    let mut last_port = 0;
    let mut logged_in = login(client, config).await;

    loop {
        if logged_in {
            match check_port_forward(config).await {
                Ok(port) => {
                    if last_port.ne(&port) {
                        if set_qb_port_value(client, config, port).await {
                            last_port = port;
                        }
                    } else {
                        debug!("Current and previous port match. No update required.")
                    }
                }
                Err(error) => {
                    error!("Unable to get port value: {}", error)
                }
            }
        } else {
            logged_in = login(client, config).await;
        }
        config.wait().await
    }
}

/// Attempts to log in and returns true if successful
async fn login(client: &Client, config: &Configuration) -> bool {
    let response = client
        .post(config.qb_login_endpoint())
        .form(&config.qb_login_parameters())
        .send()
        .await;

    match response {
        Ok(r) => {
            let status = r.status();
            if status.is_success() {
                debug!("Login successful");
                true
            } else {
                warn!("login request failed with status code: {}", status);
                false
            }
        }
        Err(e) => {
            error!("Login request error: {:?}", e);
            false
        }
    }
}
/// Attempts to set port value and returns true if successful
async fn set_qb_port_value(client: &Client, config: &Configuration, port: u16) -> bool {
    let json = get_list_port_json(port);
    let response = client
        .post(config.qb_set_preference_endpoint())
        .form(&json)
        .send()
        .await;

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

async fn check_port_forward(config: &Configuration) -> anyhow::Result<u16> {
    if ! config.port_forward_path().try_exists()? {
        warn!("Path to port forward value does nto exist");
    }
    let value = std::fs::read_to_string(config.port_forward_path())?;
    debug!("{:?}", value);
    let value = value.trim_matches(LINE_FEED);
    Ok(value.parse::<u16>()?)
}

fn get_list_port_json(port: u16) -> HashMap<String, String> {
    let mut value = HashMap::new();
    value.insert("json".into(), format!("{{listen_port:{} }}", port));
    value
}

fn log_level() -> LevelFilter {
    match std::env::var(LOG_LEVEL) {
        Ok(v) => LevelFilter::from_str(v.as_str())
            .ok()
            .unwrap_or(LevelFilter::INFO),
        Err(_) => LevelFilter::INFO,
    }
}
