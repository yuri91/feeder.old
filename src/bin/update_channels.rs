extern crate feeder;
extern crate reqwest;
#[macro_use]
extern crate quicli;
extern crate rss;
extern crate diesel;

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
                title: it.title(),
                link: it.link(),
                description: it.description(),
                author: it.author(),
                guid: it.guid().map(|g| g.value()),
                pub_date: it.pub_date(),
            };
            let inserted = feeder::queries::items::insert_if_new(&conn, &new_item)?;
            if let Some(inserted) = inserted {
                count += 1;
                for cat in it.categories() {
                    let new_cat = feeder::models::NewCategory {
                        name: cat.name(),
                        domain: cat.domain(),
                        channel_id: s.id,
                    };
                    let category = feeder::queries::categories::get_or_create(&conn, &new_cat)?;
                    let item_category = feeder::models::NewItemCategory {
                        item_id: inserted.id,
                        category_id: category.id
                    };
                    feeder::queries::items::add_category(&conn, &item_category)?;
                }
            }
        }

        println!("inserted {}",count);
    }
});

