extern crate actix;
extern crate actix_web;
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate r2d2;
extern crate serde;
extern crate serde_json;

extern crate feeder;

use actix_web::*;
use actix::prelude::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use futures::Future;

use feeder::models::{Channel, Item, NewReadItem, ReadItem, UserItem};

struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

struct GetChannels;
struct GetItems {
    chan_id: i32,
}
struct DoReadItem {
    item_id: i32,
}

impl Message for GetChannels {
    type Result = Result<Vec<Channel>, Error>;
}
impl Message for GetItems {
    type Result = Result<Vec<UserItem>, Error>;
}
impl Message for DoReadItem {
    type Result = Result<(), Error>;
}

impl Handler<GetChannels> for DbExecutor {
    type Result = Result<Vec<Channel>, Error>;

    fn handle(&mut self, _: GetChannels, _: &mut Self::Context) -> Self::Result {
        use feeder::schema::channels::dsl::*;
        use feeder::schema::subscriptions;

        let conn: &PgConnection = &self.0.get().unwrap();
        Ok(channels
            .inner_join(subscriptions::table)
            .filter(subscriptions::columns::user_id.eq(1))
            .select(feeder::schema::channels::all_columns)
            .get_results(conn)
            .map_err(|_| error::ErrorInternalServerError("Error querying channels"))?)
    }
}
impl Handler<GetItems> for DbExecutor {
    type Result = Result<Vec<UserItem>, Error>;

    fn handle(&mut self, gi: GetItems, _: &mut Self::Context) -> Self::Result {
        use feeder::schema::items::dsl::*;
        use feeder::schema::read_items;
        use feeder::schema::subscriptions;

        let conn: &PgConnection = &self.0.get().unwrap();
        Ok(items
            .inner_join(subscriptions::table.on(channel_id.eq(subscriptions::channel_id)))
            .left_join(read_items::table)
            .filter(subscriptions::user_id.eq(1))
            .filter(channel_id.eq(gi.chan_id))
            .select((
                feeder::schema::items::all_columns,
                read_items::all_columns.nullable(),
            ))
            .get_results(conn)
            .map(|v: Vec<(Item, Option<ReadItem>)>| {
                v.into_iter()
                    .map(|(i, r)| UserItem {
                        item: i,
                        read: r.is_some(),
                    })
                    .collect()
            })
            .map_err(|_| {
                error::ErrorInternalServerError(format!(
                    "Error getting items for channel {}",
                    gi.chan_id
                ))
            })?)
    }
}
impl Handler<DoReadItem> for DbExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, ri: DoReadItem, _: &mut Self::Context) -> Self::Result {
        let user_id = 1;
        let conn: &PgConnection = &self.0.get().unwrap();
        let _ = feeder::queries::read_items::get_or_create(
            conn,
            &NewReadItem {
                user_id: user_id,
                item_id: ri.item_id,
            },
        ).map_err(|_| {
            error::ErrorInternalServerError(format!(
                "Error setting item {} as read for user {}",
                ri.item_id, user_id
            ))
        })?;
        Ok(())
    }
}

struct AppState {
    db: Addr<Syn, DbExecutor>,
}

fn channels(state: State<AppState>) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(GetChannels)
        .from_err()
        .and_then(|res| match res {
            Ok(c) => Ok(HttpResponse::Ok().json(c)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
fn items(chan: Path<i32>, state: State<AppState>) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(GetItems {
            chan_id: chan.into_inner(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(i) => Ok(HttpResponse::Ok().json(i)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
fn read(it: Path<i32>, state: State<AppState>) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(DoReadItem {
            item_id: it.into_inner(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    dotenv().ok();
    env_logger::init();

    let sys = actix::System::new("feeder-api");

    let manager = ConnectionManager::<PgConnection>::new(
        std::env::var("DATABASE_URL").expect("no DATABASE_URL set"),
    );
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let addr = SyncArbiter::start(4, move || DbExecutor(pool.clone()));

    server::new(move || {
        App::with_state(AppState { db: addr.clone() })
            .middleware(middleware::Logger::default())
            .configure(|app| {
                middleware::cors::Cors::for_app(app)
                    .allowed_origin("http://localhost:8000")
                    .resource("/channels", |r| r.method(http::Method::GET).with(channels))
                    .resource("/items/{chan_id}", |r| {
                        r.method(http::Method::GET).with2(items)
                    })
                    .resource("/read/{item_id}", |r| {
                        r.method(http::Method::POST).with2(read)
                    })
                    .register()
            })
    }).bind("127.0.0.1:8888")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8888");
    let _ = sys.run();
}
