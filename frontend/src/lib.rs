#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate yew;

pub mod feeder;

#[derive(Deserialize, PartialEq, Debug)]
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

#[derive(Deserialize, PartialEq, Debug)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub domain: Option<String>,
    pub channel_id: i32,
}

#[derive(Deserialize, PartialEq, Debug)]
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

pub static BASE_URL: &'static str = "http://localhost:8888";
