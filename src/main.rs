use crate::apps::app_init;
use std::str::FromStr;
use tracing::{debug, error};
use tracing_subscriber::filter::LevelFilter;

mod apps;
mod rpc;

const LINE_FEED: char = '\n';
const LOG_LEVEL: &str = "LOG_LEVEL";

fn main() {
    tracing_subscriber::fmt().with_max_level(log_level()).init();
    if let Err(error) = run() {
        error!("{error}")
    }
}

fn run() -> anyhow::Result<()> {
    let app = app_init()?;
    let mut last_port = 0;
    let mut logged_in = app.login();

    loop {
        if logged_in {
            match app.check_port_forward() {
                Ok(port) => {
                    if last_port.ne(&port) {
                        if app.set_port(port) {
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
            logged_in = app.login();
        }
        app.wait()
    }
}

fn log_level() -> LevelFilter {
    match std::env::var(LOG_LEVEL) {
        Ok(v) => LevelFilter::from_str(v.as_str())
            .ok()
            .unwrap_or(LevelFilter::INFO),
        Err(_) => LevelFilter::INFO,
    }
}
