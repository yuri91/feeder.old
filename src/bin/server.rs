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
extern crate serde_json;

extern crate feeder;

use actix_web::*;
use actix::prelude::*;
use actix_web::dev::AsyncResult;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use futures::Future;

use feeder::models::User;
use feeder::actors::DbExecutor;
use feeder::actors::msg;

struct AppState {
    db: Addr<DbExecutor>,
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
                .send(msg::GetUser { name: name.clone() })
                .from_err()
                .and_then(move |r| match r {
                    Ok(user) => Ok(Identity { user }),
                    Err(_) => Err(error::ErrorInternalServerError(format!("Cannot find user {}", name))),
                })
        ))
    }
}

fn channels(state: State<AppState>, identity: Identity) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(msg::GetChannels{user_id: identity.user.id})
        .from_err()
        .and_then(|res| match res {
            Ok(c) => Ok(HttpResponse::Ok().json(c)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
fn items(
    get_items: Query<msg::GetItems>,
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
        .send(msg::ReadItem {
            item_id: it.into_inner(),
            user_id: identity.user.id
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
        .send(msg::ReadAllItems {
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
            .configure(|app| {
                middleware::cors::Cors::for_app(app)
                    .allowed_origin("http://localhost:1234")
                    .resource("/channels", |r| r.method(http::Method::GET).with(channels))
                    .resource("/items", |r| r.method(http::Method::GET).with(items))
                    .resource("/read/all", |r| r.method(http::Method::POST).with(read_all))
                    .resource("/read/{item_id}", |r| {
                        r.method(http::Method::POST).with(read)
                    })
           .register()
        })
    }).bind("127.0.0.1:8888")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8888");
    let _ = sys.run();
}
