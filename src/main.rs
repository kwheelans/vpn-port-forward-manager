use crate::apps::{app_init, result_to_bool};
use crate::error::Result;
use std::str::FromStr;
use tracing::{error, trace};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::time::LocalTime;

mod apps;
mod error;
mod rpc;

const LINE_FEED: char = '\n';
const LOG_LEVEL: &str = "LOG_LEVEL";

fn main() {
    tracing_subscriber::fmt()
        .with_timer(LocalTime::rfc_3339())
        .with_max_level(log_level())
        .init();
    if let Err(error) = run() {
        error!("{error}")
    }
}

fn run() -> Result<()> {
    let app = app_init()?;
    let mut last_port = 0;
    let mut logged_in = result_to_bool(app.login());

    if !logged_in {
        app.wait()
    }

    loop {
        if logged_in {
            match app.check_port_forward() {
                Ok(port) => {
                    if last_port.ne(&port) {
                        if result_to_bool(app.set_port(port)) {
                            last_port = port;
                        }
                    } else {
                        trace!("Current and previous port match. No update required.")
                    }
                }
                Err(error) => {
                    error!("Unable to get port value: {}", error)
                }
            }
        } else {
            logged_in = result_to_bool(app.login());
        }
        app.wait()
    }
}

fn log_level() -> LevelFilter {
    match std::env::var(LOG_LEVEL) {
        Ok(v) => LevelFilter::from_str(v.as_str()).unwrap_or(LevelFilter::INFO),
        Err(_) => LevelFilter::INFO,
    }
}
