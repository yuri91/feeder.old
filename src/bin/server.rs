use actix_web::*;
use ::actix::prelude::*;
use actix_web::dev::AsyncResult;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use futures::Future;

use std::sync::Arc;

use log::info;

use feeder::models::User;
use feeder::actors::{DbExecutor, GraphQLExecutor, GraphQLData, UserGraphQLData};
use feeder::actors::msg;
use feeder::graphql;

struct AppState {
    db: Addr<DbExecutor>,
    executor: Addr<GraphQLExecutor>,
}

struct Identity {
    user: User,
}
impl FromRequest<AppState> for Identity {
    type Config = ();
    type Result = AsyncResult<Identity, Error>;

    #[inline]
    fn from_request(req: &HttpRequest<AppState>, _: &Self::Config) -> Self::Result {
        //let name = match req.headers()
        //    .get("X-Forwarded-User")
        //    .ok_or_else(|| error::ErrorInternalServerError("Error querying channels"))
        //{
        //    Ok(n) => n,
        //    Err(e) => return AsyncResult::err(e),
        //};
        //let name = match name.to_str().map_err(|_| {
        //    error::ErrorInternalServerError("Header value contains invalid characters")
        //}) {
        //    Ok(n) => n.to_owned(),
        //    Err(e) => return AsyncResult::err(e),
        //};
        let name = "yuri".to_owned();
        info!("X-Forwarded-User: {}", name);
        AsyncResult::future(Box::new(
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

fn graphiql(_req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let html = juniper::http::graphiql::graphiql_source("http://127.0.0.1:8888/graphql");
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

fn graphql(st: State<AppState>, data: Json<GraphQLData>, identity: Identity) -> FutureResponse<HttpResponse> {
    let msg = UserGraphQLData { user: identity.user, data: data.0};
    st.executor
        .send(msg)
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(user)),
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

    let db_pool = pool.clone();
    let db_addr = SyncArbiter::start(4, move || DbExecutor(db_pool.clone()));
    let graphql_addr = SyncArbiter::start(4, move || GraphQLExecutor{schema: Arc::new(graphql::create_schema()), db: pool.clone()});

    server::new(move || {
        App::with_state(AppState { db: db_addr.clone(), executor: graphql_addr.clone()})
            .middleware(middleware::Logger::default())
            .configure(|app| {
                middleware::cors::Cors::for_app(app)
                    .allowed_origin("http://localhost:8888")
                    .supports_credentials()
                    .resource("/channels", |r| r.method(http::Method::GET).with(channels))
                    .resource("/items", |r| r.method(http::Method::GET).with(items))
                    .resource("/read/all", |r| r.method(http::Method::POST).with(read_all))
                    .resource("/read/{item_id}", |r| {
                        r.method(http::Method::POST).with(read)
                    })
                    .resource("/graphql", |r| r.method(http::Method::POST).with(graphql))
                    .resource("/graphiql", |r| r.method(http::Method::GET).h(graphiql))
           .register()
        })
    }).bind("127.0.0.1:8888")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8888");
    let _ = sys.run();
}
