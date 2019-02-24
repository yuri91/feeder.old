use juniper::graphql_object;
use juniper::FieldResult;
use juniper::RootNode;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::models::*;
use crate::queries;

pub struct Context {
    pub db: Pool<ConnectionManager<PgConnection>>,
    pub user: User,
}

impl juniper::Context for Context {}

graphql_object!(Channel: Context |&self| {
    field id() -> i32 {
        self.id
    }
    field title() -> &str {
        &self.title
    }
    field link() -> &str {
        &self.link
    }
    field description() -> &str {
        &self.description
    }
    field source() -> &str {
        &self.source
    }
    field image() -> Option<&str> {
        self.image.as_ref().map(|s| s.as_ref())
    }
    field ttl() -> Option<i32> {
        self.ttl
    }
});

graphql_object!(UserItem: Context as "Item" |&self| {
    field id() -> i32 {
        self.item.id
    }
    field title() -> &str {
        &self.item.title
    }
    field channel(&executor) -> FieldResult<Channel> {
        let conn: &PgConnection = &executor.context().db.get().unwrap();
        Ok(queries::channels::get(conn, self.item.channel_id)?.unwrap())
    }
    field link() -> &str {
        &self.item.link
    }
    field description() -> &str {
        &self.item.description
    }
    field pub_date() -> String {
        self.item.pub_date.to_string()
    },
    field guid() -> Option<&str> {
        self.item.guid.as_ref().map(|s| s.as_ref())
    }
    field read() -> bool {
        self.read
    }
});

pub struct Query;
graphql_object!(Query: Context |&self| {
    field items(&executor) -> FieldResult<Vec<UserItem>> {
        let conn: &PgConnection = &executor.context().db.get().unwrap();
        Ok(queries::items::get_all_for(conn, executor.context().user.id)?)
    }
    field subscriptions(&executor) -> FieldResult<Vec<Channel>> {
        let conn: &PgConnection = &executor.context().db.get().unwrap();
        Ok(queries::channels::get_all_for(conn, executor.context().user.id)?)
    }
});

pub struct Mutation;
graphql_object!(Mutation: Context |&self| {
    field read(&executor, id: i32) -> FieldResult<()> {
        let conn: &PgConnection = &executor.context().db.get().unwrap();
        queries::read_items::read(conn, executor.context().user.id, id)?;
        Ok(())
    }
    field read_all(&executor) -> FieldResult<()> {
        let conn: &PgConnection = &executor.context().db.get().unwrap();
        queries::read_items::read_all(conn, executor.context().user.id)?;
        Ok(())
    }
    field subscribe(&executor, url: String) -> FieldResult<Channel> {
        let conn: &PgConnection = &executor.context().db.get().unwrap();

        let resp = reqwest::get(&url)?;
        let channel = rss::Channel::read_from(std::io::BufReader::new(resp)).expect("Invalid xml file");

        let channel = NewChannel {
            title: channel.title(),
            link: channel.link(),
            description: channel.description(),
            source: &url,
            image: channel.image().map(|i| i.url()),
            ttl: channel.ttl().map(|t| t.parse().expect("Invalid TTL field")),
        };
        let channel = queries::channels::get_or_create(&conn, &channel)?;
        Ok(channel)
    }
});

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}