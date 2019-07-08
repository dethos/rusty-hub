use super::schema::subscriptions;
use super::schema::subscriptions::dsl::*;
use diesel::prelude::*;
use models::*;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::time::{SystemTime, UNIX_EPOCH};
use utils::{setup_logging, Pool};

pub fn handle_subscription(db: &Pool, data: &HashMap<String, String>) -> bool {
    let log = setup_logging();
    let mode;
    let req_callback;
    let req_topic;
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
        Some(value) => req_callback = value,
        None => return false,
    }

    match data.get("hub.topic") {
        Some(value) => req_topic = value,
        None => return false,
    }

    debug!(
        log,
        "Mode: {}, Callback: {}, topic: {}", mode, req_callback, req_topic
    );

    if mode == &"subscribe" {
        let subscription = NewSubscription {
            callback: req_callback,
            topic: req_topic,
            sec: "",
            created_at: &now,
            expires_at: &now,
        };

        diesel::insert_into(subscriptions::table)
            .values(&subscription)
            .execute(&conn)
            .expect("Error saving new subscription");

        debug!(
            log,
            "Subscription created. Callback {}. Topic {}",
            subscription.callback,
            subscription.topic
        );
        return true;
    } else if mode == &"unsubscribe" {
        let sub = subscriptions
            .filter(callback.eq(req_callback))
            .filter(topic.eq(req_topic));
        diesel::delete(sub)
            .execute(&conn)
            .expect("Error removing subscription");
        debug!(
            log,
            "Subscription removed. Callback {}. Topic {}", req_callback, req_topic
        );
        return true;
    } else {
        debug!(log, "Wrong method.");
        return false;
    }
}

#[cfg(test)]
mod tests {}
