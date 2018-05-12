extern crate feeder;
extern crate reqwest;
#[macro_use]
extern crate quicli;
extern crate rss;

use quicli::prelude::*;

use std::io::BufReader;

#[derive(Debug, StructOpt)]
struct Cli {
    /// Upstream URL for the channel
    #[structopt(long = "url", short = "u")]
    url: String,
}

main!(|args: Cli| {
    let resp = reqwest::get(&args.url)?;
    let channel = rss::Channel::read_from(BufReader::new(resp)).expect("Invalid xml file");

    let conn = feeder::establish_connection();

    let channel = feeder::models::NewChannel {
        title: channel.title(),
        link: channel.link(),
        description: channel.description(),
        source: &args.url,
        image: channel.image().map(|i| i.url()),
        ttl: channel.ttl().map(|t| t.parse().expect("Invalid TTL field")),
    };
    let channel = feeder::queries::channels::get_or_create(&conn, &channel)?;

    println!("channel {} added", channel.title);

    let user = feeder::models::NewUser{name: "yuri"};
    let user = feeder::queries::users::get_or_create(&conn, &user)?;

    let sub = feeder::models::NewSubscription {
        user_id: user.id,
        channel_id: channel.id,
    };
    let _ = feeder::queries::subscriptions::get_or_create(&conn, &sub)?;

    println!("user {} subscribed to channel", user.name);
});

