use super::schema::subscriptions;
use diesel::prelude::*;
use models::NewSubscription;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::time::{SystemTime, UNIX_EPOCH};
use utils::{setup_logging, Pool};

pub fn handle_subscription(db: &Pool, data: &HashMap<String, String>) -> bool {
    let log = setup_logging();
    let mode;
    let callback;
    let topic;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Invalid Time")
        .as_secs();
    let now = i32::try_from(timestamp).ok().unwrap();
    let conn = db.get().unwrap();

    match data.get("hub.mode") {
        Some(value) => mode = value,
        None => return false,
    }

    match data.get("hub.callback") {
        Some(value) => callback = value,
        None => return false,
    }

    match data.get("hub.topic") {
        Some(value) => topic = value,
        None => return false,
    }

    debug!(
        log,
        "Mode: {}, Callback: {}, topic: {}", mode, callback, topic
    );

    if mode == &"subscribe" {
        let subscription = NewSubscription {
            callback: callback,
            topic: topic,
            sec: "",
            created_at: &now,
            expires_at: &now,
        };

        diesel::insert_into(subscriptions::table)
            .values(&subscription)
            .execute(&conn)
            .expect("Error saving new subscription");
        debug!(log, "Subscription created.");
        return true;
    } else if mode == &"unsubscribe" {
        return true;
    } else {
        debug!(log, "Wrong method.");
        return false;
    }
}

#[cfg(test)]
mod tests {}
