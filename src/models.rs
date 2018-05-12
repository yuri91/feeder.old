use super::schema::channels;
use super::schema::items;
use super::schema::users;
use super::schema::subscriptions;
use super::schema::read_items;
use super::chrono::NaiveDateTime;


#[derive(Identifiable, Queryable, Serialize, PartialEq, Debug)]
#[table_name = "channels"]
pub struct Channel {
    pub id: i32,
    pub title: String,
    pub link: String,
    pub description: String,
    pub source: String,
    pub image: Option<String>,
    pub ttl: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "channels"]
pub struct NewChannel<'a> {
    pub title: &'a str,
    pub link: &'a str,
    pub description: &'a str,
    pub source: &'a str,
    pub image: Option<&'a str>,
    pub ttl: Option<i32>,
}

#[derive(Identifiable, Queryable, Associations, Serialize, PartialEq, Debug)]
#[belongs_to(Channel)]
#[table_name = "items"]
pub struct Item {
    pub id: i32,
    pub channel_id: i32,
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: NaiveDateTime,
    pub guid: Option<String>,
}

#[derive(Insertable)]
#[table_name = "items"]
pub struct NewItem<'a> {
    pub channel_id: i32,
    pub title: &'a str,
    pub link: &'a str,
    pub description: &'a str,
    pub pub_date: NaiveDateTime,
    pub guid: Option<&'a str>,
}

#[derive(Serialize, PartialEq, Debug)]
pub struct UserItem {
    #[serde(flatten)]
    pub item: Item,
    pub read: bool,
}

#[derive(Identifiable, Queryable, Associations, Serialize, PartialEq, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User)]
#[belongs_to(Channel)]
#[table_name = "subscriptions"]
pub struct Subscription {
    pub id: i32,
    pub user_id: i32,
    pub channel_id: i32,
}

#[derive(Insertable)]
#[table_name = "subscriptions"]
pub struct NewSubscription {
    pub user_id: i32,
    pub channel_id: i32,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User)]
#[belongs_to(Item)]
#[table_name = "read_items"]
pub struct ReadItem {
    pub id: i32,
    pub user_id: i32,
    pub item_id: i32,
}

#[derive(Insertable)]
#[table_name = "read_items"]
pub struct NewReadItem {
    pub user_id: i32,
    pub item_id: i32,
}
