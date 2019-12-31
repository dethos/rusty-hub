use super::schema::subscriptions;
use super::schema::subscriptions::dsl::*;
use crate::models::*;
use crate::utils::{setup_logging, Pool};
use diesel::prelude::*;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::time::{SystemTime, UNIX_EPOCH};

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
    } else {
        debug!(log, "Wrong method.");
        return false;
    }

    return true;
}

pub fn handle_publication(db: &Pool, data: &HashMap<String, String>) -> bool {
    // Implement later
    true
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

        assert_eq!(handle_subscription(&pool, &hashmap), false);
    }
}
