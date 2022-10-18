// #![allow(dead_code, unused)]
#[macro_use] extern crate diesel;

use actix_cors::Cors;
use actix_web::{
    middleware::Logger,
    cookie::Key, web,
    App, HttpServer};
use actix_identity::{IdentityMiddleware};
use actix_session::{storage::RedisSessionStore, SessionMiddleware};

mod auth;
mod db;
mod model;
mod schema;
mod action;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server().await
}

pub async fn server() -> std::io::Result<()> {
    let secret_key = Key::generate();
    let redis_store = RedisSessionStore::new("redis://redis:6379")
        .await
        .unwrap();

    let pool = db::establish_connection();

    let server = HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::builder(
                    redis_store.clone(),
                    secret_key.clone()
                )
                .cookie_secure(false)
                .build()
            )
            .wrap(cors)
            .wrap(Logger::default())
            .configure(routes)
    });
    server
        .bind("0.0.0.0:8080")?
        .run()
        .await
        .expect("Error in build httpserver");

    Ok(())
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(auth::index))
        .route("/login", web::post().to(auth::login))
        .route("/logout", web::post().to(auth::logout))
        .service(
            web::scope("/api/v1")
                .route("/user_info", web::get().to(auth::get_user_info))
                .route("/user_list", web::get().to(action::get_user_list))
        );
}
