use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use slog::Drain;

use std::collections::HashMap;
use url::Url;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub struct AppState {
    pub log: slog::Logger,
    pub db: Pool,
}

pub fn setup_logging() -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!())
}

pub fn validate_parsed_data(parameters: &HashMap<String,String>) -> bool {
    let callback;
    let mode;
    let topic;

    match parameters.get("hub.callback") {
        Some(value) => callback = value,
        None => return false,
    };

    match parameters.get("hub.mode") {
        Some(value) => mode = value,
        None => return false,
    };

    match parameters.get("hub.topic") {
        Some(value) => topic = value,
        None => return false,
    };

    if mode != &"subscribe" && mode != &"unsubscribe" {
        debug!(setup_logging(), "Invalid Method: {}", mode);
        return false;
    }

    match Url::parse(callback) {
        Ok(value) => debug!(setup_logging(), "Valid Callback: {}", value),
        Err(_) => return false,
    }

    match Url::parse(topic) {
        Ok(value) => debug!(setup_logging(), "Valid Topic: {}", value),
        Err(_) => return false,
    }
    true
}
