extern crate feeder;
extern crate reqwest;
#[macro_use]
extern crate quicli;
extern crate rss;
extern crate diesel;
extern crate chrono;

use chrono::{DateTime, FixedOffset};

use diesel::prelude::*;

use std::io::BufReader;

main!({
    use feeder::schema::channels::dsl::*;

    let conn = feeder::establish_connection();
    let sources = channels.load::<feeder::models::Channel>(&conn)?;

    let mut count = 0;
    for s in sources {
        let resp = reqwest::get(&s.source)?;
        let channel = rss::Channel::read_from(BufReader::new(resp)).expect("Invalid xml file");
        for it in channel.items() {
            let new_item = feeder::models::NewItem {
                channel_id: s.id,
                title: match it.title() {
                    Some(t) => t,
                    None => continue,
                },
                link: match it.link() {
                    Some(l) => l,
                    None => continue,
                },
                description: match it.description() {
                    Some(d) => d,
                    None => continue,
                },
                guid: it.guid().map(|g| g.value()),
                pub_date: it.pub_date().and_then(|d| {
                    match DateTime::<FixedOffset>::parse_from_rfc2822(d) {
                        Ok(d) => Some(d.naive_utc()),
                        Err(_) => None,
                    }
                }).unwrap_or(chrono::offset::Utc::now().naive_utc()),
            };
            let inserted = feeder::queries::items::insert_if_new(&conn, &new_item)?;
            if inserted.is_some() {
                count += 1;
            }
        }

        println!("inserted {} items for {}", count, s.title);
    }
});

