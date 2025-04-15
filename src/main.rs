use actix_web::{web, App, HttpServer};
use actix_files as fs;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use tera::Tera;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
mod models;
mod schema;
mod routes;
mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化数据库连接池
    let manager = ConnectionManager::<SqliteConnection>::new("language_delay.db");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    // 初始化 Tera 模板引擎
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };
    // 初始化会话
    let secret_key = actix_web::cookie::Key::generate();

    // 确保 uploads 目录存在
    std::fs::create_dir_all("uploads").unwrap();

    println!("Server running at http://127.0.0.1:8000");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera.clone()))
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .service(fs::Files::new("/static", "static").show_files_listing())
            .service(routes::index)
            .service(routes::education)
            .service(routes::screening)
            .service(routes::community)
            .service(routes::create_post)
            .service(routes::resources)
            .service(routes::login_page)
            .service(routes::login)
            .service(routes::register_page)
            .service(routes::register)
            .service(routes::logout)
            .service(routes::upload_video)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}