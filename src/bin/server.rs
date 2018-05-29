extern crate actix;
extern crate actix_web;
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate futures;
extern crate r2d2;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate feeder;

use actix_web::*;
use actix::prelude::*;
use actix_web::dev::AsyncResult;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use futures::Future;

use feeder::models::{Channel, Item, NewReadItem, NewUser, ReadItem, User, UserItem};

struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

struct GetIdentity {
    name: String,
}
struct GetChannels {
    user_id: i32,
}
#[derive(Deserialize)]
struct GetItems {
    #[serde(skip)]
    user_id: i32,
    from_id: i32,
    to_id: i32,
    max_items: i32,
}
struct DoReadItem {
    item_id: i32,
    user_id: i32,
}

impl Message for GetIdentity {
    type Result = Result<Identity, Error>;
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

impl Handler<GetIdentity> for DbExecutor {
    type Result = Result<Identity, Error>;

    fn handle(&mut self, gu: GetIdentity, _: &mut Self::Context) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();
        feeder::queries::users::get_or_create(conn, &NewUser { name: &gu.name })
            .map(|user| Identity { user })
            .map_err(|_| error::ErrorInternalServerError(format!("Error getting user {}", gu.name)))
    }
}
impl Handler<GetChannels> for DbExecutor {
    type Result = Result<Vec<Channel>, Error>;

    fn handle(&mut self, gc: GetChannels, _: &mut Self::Context) -> Self::Result {
        use feeder::schema::channels::dsl::*;
        use feeder::schema::subscriptions;

        let conn: &PgConnection = &self.0.get().unwrap();
        Ok(channels
            .inner_join(subscriptions::table)
            .filter(subscriptions::columns::user_id.eq(gc.user_id))
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
            .filter(subscriptions::user_id.eq(gi.user_id))
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
                    "Error getting items for user {}",
                    gi.user_id
                ))
            })?)
    }
}
impl Handler<DoReadItem> for DbExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, ri: DoReadItem, _: &mut Self::Context) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();
        if ri.item_id < 0 {
            let _ = feeder::queries::read_items::read_all(conn, ri.user_id).map_err(|_| {
                error::ErrorInternalServerError(format!(
                    "Error setting item {} as read for user {}",
                    ri.item_id, ri.user_id
                ))
            })?;
        } else {
            let _ = feeder::queries::read_items::get_or_create(
                conn,
                &NewReadItem {
                    user_id: ri.user_id,
                    item_id: ri.item_id,
                },
            ).map_err(|_| {
                error::ErrorInternalServerError(format!(
                    "Error setting item {} as read for user {}",
                    ri.item_id, ri.user_id
                ))
            })?;
        }
        Ok(())
    }
}

struct AppState {
    db: Addr<Syn, DbExecutor>,
}

struct Identity {
    user: User,
}
impl FromRequest<AppState> for Identity {
    type Config = ();
    type Result = AsyncResult<Identity, Error>;

    #[inline]
    fn from_request(req: &HttpRequest<AppState>, _: &Self::Config) -> Self::Result {
        let name = match req.headers()
            .get("X-Forwarded-User")
            .ok_or(error::ErrorInternalServerError("Error querying channels"))
        {
            Ok(n) => n,
            Err(e) => return AsyncResult::err(e),
        };
        let name = match name.to_str().map_err(|_| {
            error::ErrorInternalServerError("Header value contains invalid characters")
        }) {
            Ok(n) => n.to_owned(),
            Err(e) => return AsyncResult::err(e),
        };
        info!("X-Forwarded-User: {}", name);
        AsyncResult::async(Box::new(
            req.state()
                .db
                .send(GetIdentity { name })
                .from_err()
                .and_then(|r| r),
        ))
    }
}

fn channels(state: State<AppState>, identity: Identity) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(GetChannels{user_id: identity.user.id})
        .from_err()
        .and_then(|res| match res {
            Ok(c) => Ok(HttpResponse::Ok().json(c)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
fn items(
    get_items: Query<GetItems>,
    state: State<AppState>,
    identity: Identity,
) -> FutureResponse<HttpResponse> {
    let mut get_items = get_items.into_inner();
    get_items.user_id = identity.user.id;
    state
        .db
        .send(get_items)
        .from_err()
        .and_then(|res| match res {
            Ok(i) => Ok(HttpResponse::Ok().json(i)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
fn read(it: Path<i32>, state: State<AppState>, identity: Identity) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(DoReadItem {
            item_id: it.into_inner(),
            user_id: identity.user.id,
        })
        .from_err()
        .and_then(|res| match res {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
fn read_all(state: State<AppState>, identity: Identity) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(DoReadItem {
            item_id: -1,
            user_id: identity.user.id,
        })
        .from_err()
        .and_then(|res| match res {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info,feeder=info,server=info");
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
            //.configure(|app| {
                //middleware::cors::Cors::for_app(app)
                //    .allowed_origin("http://localhost:8000")
                    .resource("/channels", |r| r.method(http::Method::GET).with2(channels))
                    .resource("/items", |r| r.method(http::Method::GET).with3(items))
                    .resource("/read/all", |r| r.method(http::Method::POST).with2(read_all))
                    .resource("/read/{item_id}", |r| {
                        r.method(http::Method::POST).with3(read)
                    })
        //   .register()
        //})
    }).bind("127.0.0.1:8888")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8888");
    let _ = sys.run();
}
