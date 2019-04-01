table! {
    subscriptions (id) {
        id -> Integer,
        callback -> Text,
        topic -> Text,
        sec -> Text,
        created_at -> Integer,
        expires_at -> Integer,
    }
}
