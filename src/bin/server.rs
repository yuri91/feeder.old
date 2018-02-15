#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;

extern crate diesel;

extern crate r2d2;
extern crate r2d2_diesel;

extern crate dotenv;

#[macro_use]
extern crate serde_derive;

extern crate feeder;

use feeder::models::{Channel, Item};

use rocket_contrib::Json;
use diesel::QueryResult;

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

use dotenv::dotenv;

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

// For the convenience of using an &DbConn as an &SqliteConnection.
impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn init_pool() -> Pool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(manager).expect("db pool error")
}

#[get("/items/<chan_id>")]
fn items(conn: DbConn, chan_id: i32) -> QueryResult<Json<Vec<Item>>> {
    use feeder::schema::items::dsl::*;
    use diesel::prelude::*;

    let its = items
        .filter(channel_id.eq(chan_id))
        .get_results(&*conn)?;
    Ok(Json(its))
}

#[get("/channels")]
fn channels(conn: DbConn) -> QueryResult<Json<Vec<Channel>>> {
    use feeder::schema::channels::dsl::*;
    use diesel::prelude::*;

    channels.get_results(&*conn).map(|c| Json(c))
}

fn main() {
    dotenv().ok();

    let cors = rocket_cors::Cors::default();
    rocket::ignite()
        .manage(init_pool())
        .attach(cors)
        .mount("/", routes![channels, items])
        .launch();
}
