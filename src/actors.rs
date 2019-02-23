use ::actix::prelude::*;
use ::diesel::pg::PgConnection;
use ::diesel::QueryResult;
use diesel::r2d2::{ConnectionManager, Pool};

use super::models::*;
use super::queries;

pub mod msg {
    use ::actix::prelude::*;
    use ::diesel::QueryResult;
    use serde_derive::Deserialize;

    use super::super::models::*;

    #[derive(Clone)]
    pub struct GetUser {
        pub name: String,
    }
    #[derive(Clone, Copy)]
    pub struct GetChannels {
        pub user_id: i32,
    }
    #[derive(Deserialize, Clone, Copy)]
    pub struct GetItems {
        #[serde(skip)]
        pub user_id: i32,
        pub from_id: i32,
        pub to_id: i32,
        pub max_items: i32,
    }
    #[derive(Clone, Copy)]
    pub struct ReadItem {
        pub item_id: i32,
        pub user_id: i32,
    }
    #[derive(Clone, Copy)]
    pub struct ReadAllItems {
        pub user_id: i32,
    }

    impl Message for GetUser {
        type Result = QueryResult<User>;
    }
    impl Message for GetChannels {
        type Result = QueryResult<Vec<Channel>>;
    }
    impl Message for GetItems {
        type Result = QueryResult<Vec<UserItem>>;
    }
    impl Message for ReadItem {
        type Result = QueryResult<()>;
    }
    impl Message for ReadAllItems {
        type Result = QueryResult<()>;
    }
}

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<msg::GetUser> for DbExecutor {
    type Result = QueryResult<User>;

    fn handle(&mut self, gu: msg::GetUser, _: &mut Self::Context) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();
        queries::users::get_or_create(conn, &NewUser { name: &gu.name })
    }
}
impl Handler<msg::GetChannels> for DbExecutor {
    type Result = QueryResult<Vec<Channel>>;

    fn handle(&mut self, gc: msg::GetChannels, _: &mut Self::Context) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();
        queries::channels::get_all_for(conn, gc.user_id)
    }
}
impl Handler<msg::GetItems> for DbExecutor {
    type Result = QueryResult<Vec<UserItem>>;

    fn handle(&mut self, gi: msg::GetItems, _: &mut Self::Context) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();
        queries::items::get_all_for(conn, gi.user_id)
    }
}
impl Handler<msg::ReadItem> for DbExecutor {
    type Result = QueryResult<()>;

    fn handle(&mut self, ri: msg::ReadItem, _: &mut Self::Context) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();
        queries::read_items::read(conn, ri.user_id, ri.item_id,)
    }
}
impl Handler<msg::ReadAllItems> for DbExecutor {
    type Result = QueryResult<()>;

    fn handle(&mut self, ri: msg::ReadAllItems, _: &mut Self::Context) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();
        queries::read_items::read_all(conn, ri.user_id)
    }
}
