use juniper::graphql_object;
use juniper::FieldResult;
use juniper::RootNode;
use diesel::pg::PgConnection;
use diesel::QueryResult;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::models::{User, UserItem, };
use crate::queries;

pub struct Context {
    pub db: Pool<ConnectionManager<PgConnection>>,
    pub user: User,
}

impl juniper::Context for Context {}

graphql_object!(UserItem: Context |&self| {
    field id() -> i32 {
        self.item.id
    }
    field title() -> &str {
        &self.item.title
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
});

pub struct Mutation;
graphql_object!(Mutation: Context |&self| {
});

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}