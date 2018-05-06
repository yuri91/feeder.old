use super::schema::channels;
use super::schema::items;
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
