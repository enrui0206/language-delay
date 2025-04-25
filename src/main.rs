use actix_web::{web, App, HttpServer};
use actix_files as fs;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use tera::Tera;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::path::PathBuf;
use winres::WindowsResource;

mod models;
mod schema;
mod routes;
mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get the base directory (project root)
    let base_dir = std::env::current_exe()?
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    println!("Base directory: {}", base_dir.display());

        // Create absolute paths..
    /*if cfg!(target_os = "windows") {
        if cfg!(target_os = "windows") {
            let res = winres::WindowsResource::new()
                .set_icon("icon.ico") // optional
                .compile();

            if let Err(e) = res {
                println!("cargo:warning=winres error: {:?}", e);
            }
        }
    }*/
    // Create absolute paths
    let db_path = base_dir.join("language_delay.db");
    let templates_path = base_dir.join("templates/**/*");
    let static_path = base_dir.join("static");
    let uploads_path = base_dir.join("uploads");

    // initialize database connection pool
    let manager = ConnectionManager::<SqliteConnection>::new(db_path.to_str().unwrap());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    // initialize Tera template engine
    let tera = match Tera::new(templates_path.to_str().unwrap()) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };

    // Initialize session secret key
    let secret_key = actix_web::cookie::Key::generate();

    // ensures uploads directory exists
    std::fs::create_dir_all(&uploads_path).unwrap();

    println!("Server running at http://127.0.0.1:8000");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera.clone()))
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .service(fs::Files::new("/static", &static_path).show_files_listing())
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