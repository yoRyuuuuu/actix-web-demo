use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;

mod db;
mod errors;
mod handler;
mod model;
mod state;

static JWT_KEY: Lazy<String> =
    Lazy::new(|| env::var("JWT_KEY").expect("failed to load JWT_KEY variable"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("failed to read .env file");

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let db_url = env::var("DATABASE_URL").expect("failed to load DATABASE_URL variable");

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await
        .expect("failed to create postgres pool");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("failed to migrate");

    let state = web::Data::new(Arc::new(state::State { pool }));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(state.clone())
            .service(handler::signup)
            .service(handler::login)
            .service(handler::home)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
