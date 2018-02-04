use super::schema::categories;
use super::schema::channels;
use super::schema::items;
use super::schema::items_categories;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "channels"]
pub struct Channel {
    pub id: i32,
    pub title: String,
    pub link: String,
    pub description: String,
    pub source: String,
    pub language: Option<String>,
    pub copyright: Option<String>,
    pub pub_date: Option<String>,
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
    pub language: Option<&'a str>,
    pub copyright: Option<&'a str>,
    pub pub_date: Option<&'a str>,
    pub image: Option<&'a str>,
    pub ttl: Option<i32>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Channel)]
#[table_name = "categories"]
pub struct Category {
    pub id: i32,
    pub channel_id: i32,
    pub name: String,
    pub domain: Option<String>,
}

#[derive(Insertable)]
#[table_name = "categories"]
pub struct NewCategory<'a> {
    pub channel_id: i32,
    pub name: &'a str,
    pub domain: Option<&'a str>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Channel)]
#[table_name = "items"]
pub struct Item {
    pub id: i32,
    pub channel_id: i32,
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub guid: Option<String>,
    pub pub_date: Option<String>,
}

#[derive(Insertable)]
#[table_name = "items"]
pub struct NewItem<'a> {
    pub channel_id: i32,
    pub title: Option<&'a str>,
    pub link: Option<&'a str>,
    pub description: Option<&'a str>,
    pub author: Option<&'a str>,
    pub guid: Option<&'a str>,
    pub pub_date: Option<&'a str>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Item)]
#[belongs_to(Category)]
#[table_name = "items_categories"]
pub struct ItemCategory {
    pub id: i32,
    pub item_id: i32,
    pub category_id: i32,
}

#[derive(Insertable)]
#[table_name = "items_categories"]
pub struct NewItemCategory {
    pub item_id: i32,
    pub category_id: i32,
}

