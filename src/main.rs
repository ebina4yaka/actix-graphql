use crate::db::loaders::Loaders;
use crate::db::manager::new_pool;
use crate::graphql::schema::{create_schema, Context, Schema};
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenv::dotenv;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::sync::Arc;

#[macro_use]
extern crate diesel;

pub mod db;
pub mod graphql;
mod schema;

pub async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut rt = futures::executor::LocalPool::new();
        let ctx = &Context {
            pool: pool.clone(),
            loaders: Loaders::new(&pool),
        };
        let graphql_res = data.execute(&st, ctx);
        let res = rt.run_until(graphql_res);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

pub async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:3000/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn hello_world() -> impl Responder {
    "Hello World!"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let pool = match new_pool() {
        Ok(pool) => pool,
        Err(e) => panic!(e.to_string()),
    };

    let schema = std::sync::Arc::new(create_schema());

    let mut server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(hello_world))
            .route("/graphiql", web::get().to(graphiql))
            .route("/graphql", web::post().to(graphql))
    });

    server = server.bind("127.0.0.1:3000").unwrap();
    server.run().await
}
