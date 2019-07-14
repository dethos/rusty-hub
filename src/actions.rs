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
    let mode = data.get("hub.mode").expect("Mode not provided");
    let req_callback = data.get("hub.callback").expect("Callback not provided");
    let req_topic = data.get("hub.topic").expect("Topic not provided");
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Invalid Time")
        .as_secs();
    let now = i32::try_from(timestamp)
        .ok()
        .expect("Unable to calculate time");
    let conn = db.get().expect("Unable to grab a DB connection");

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
mod tests {
    use super::*;
    use diesel::r2d2::{self, ConnectionManager};
    use std::collections::HashMap;

    #[test]
    #[should_panic]
    fn test_handle_subscription_bad_data() {
        let manager = ConnectionManager::<SqliteConnection>::new("test.db");
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        let hashmap = HashMap::new();

        handle_subscription(&pool, &hashmap);
    }
}
