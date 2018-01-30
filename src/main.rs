extern crate rss;

use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("test_xml/rep.xml").unwrap();
    let channel = rss::Channel::read_from(BufReader::new(file)).unwrap();
    println!("title: {}", channel.title());
    println!("link: {}", channel.link());
    println!("description: {}", channel.description());
    println!("language: {:?}", channel.language());
    println!("copyright: {:?}", channel.copyright());
    println!("pub_date: {:?}", channel.pub_date());
    println!("last_build_date: {:?}", channel.last_build_date());
    println!("categories: {:?}", channel.categories());
    println!("ttl: {:?}", channel.ttl());
    println!("image: {:?}", channel.image());
    println!("text_input: {:?}", channel.text_input());
    for item in channel.items() {
        println!("{:?}", item);
        break;
    }
}
