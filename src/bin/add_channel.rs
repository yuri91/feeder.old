extern crate feeder;
#[macro_use]
extern crate quicli;
extern crate rss;

use quicli::prelude::*;

use std::fs::File;
use std::io::BufReader;

#[derive(Debug, StructOpt)]
struct Cli {
    /// Xml file for the channel
    #[structopt(long = "file", short = "f")]
    file: String,
}

main!(|args: Cli| {
    let file = File::open(&args.file)?;
    let channel = rss::Channel::read_from(BufReader::new(file)).expect("Invalid xml file");
    let conn = feeder::establish_connection();

    let new_channel = feeder::models::NewChannel {
        title: channel.title(),
        link: channel.link(),
        description: channel.description(),
        language: channel.language(),
        copyright: channel.copyright(),
        pub_date: channel.pub_date(),
        image: channel.image().map(|i| i.url()),
        ttl: channel.ttl().map(|t| t.parse().expect("Invalid TTL field")),
    };
    let inserted = feeder::queries::channels::get_or_create(&conn, &new_channel)?;

    println!("{:?}", inserted);
});

