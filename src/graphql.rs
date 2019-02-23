use juniper::graphql_object;
use juniper::FieldResult;
use juniper::RootNode;
use juniper::EmptyMutation;
use diesel::pg::PgConnection;
use diesel::QueryResult;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::models::{User, Item, };

pub struct DbContext(pub Pool<ConnectionManager<PgConnection>>);

impl juniper::Context for DbContext {}

graphql_object!(User: DbContext |&self| {
    field id() -> i32 {
        self.id
    }
    field name() -> &str {
        &self.name
    }
});

pub struct Query;
graphql_object!(Query: DbContext |&self| {
    field user(&executor, id: i32) -> User {
        User{id: 1, name: "aaa".to_owned()}
    }
});

pub struct Mutation;
graphql_object!(Mutation: DbContext |&self| {
});

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}