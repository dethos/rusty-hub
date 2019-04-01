use super::schema::posts;

#[derive(Queryable)]
pub struct Subscription {
    pub id: i32,
    pub callback: String,
    pub topic: String,
    pub sec: String,
    pub created_at: i32,
    pub expires_at: i32
}

#[derive(Insertable)]
#[table_name="subscriptions"]
pub struct NewSubscription<'a> {
    pub callback: &'a str,
    pub topic: &'a str,
    pub sec: &'a str,
    pub created_at: &'a i32,
    pub expires_at: &'a i32
}
