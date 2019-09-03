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

pub fn validate_parsed_data(parameters: &HashMap<String, String>) -> Result<(), String> {
    let callback;
    let mode;
    let topic;

    match parameters.get("hub.callback") {
        Some(value) => callback = value,
        None => return Err("No hub.callback specified".to_owned()),
    };

    match parameters.get("hub.mode") {
        Some(value) => mode = value,
        None => return Err("No hub.mode specified".to_owned()),
    };

    match parameters.get("hub.topic") {
        Some(value) => topic = value,
        None => return Err("No hub.topicspecified".to_owned()),
    };

    if mode != &"subscribe" && mode != &"unsubscribe" {
        return Err(format!("Invalid Method: {}", mode));
    }

    match Url::parse(callback) {
        Ok(value) => debug!(setup_logging(), "Valid Callback: {}", value),
        Err(_) => return Err("hub.callback is not a valid URL".to_owned()),
    };

    match Url::parse(topic) {
        Ok(value) => debug!(setup_logging(), "Valid Topic: {}", value),
        Err(_) => return Err("hub.topic is not a valid URL".to_owned()),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_validate_parsed_data_is_valid() {
        let mut params = HashMap::new();
        params.insert("hub.callback".to_string(), "http://example.com".to_string());
        params.insert("hub.topic".to_string(), "http://example2.com".to_string());
        params.insert("hub.mode".to_string(), "subscribe".to_string());

        let result = validate_parsed_data(&params);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_validate_parsed_data_is_invalid() {
        let params = HashMap::new();
        let result = validate_parsed_data(&params);
        assert_eq!(result.is_ok(), false);
    }
}
