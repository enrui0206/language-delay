use actix_web::{get, post, web, HttpResponse, Responder};
use actix_session::Session;
use tera::Tera;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use crate::models::{User, NewUser, Post, NewPost};
use crate::schema::users::dsl::{users, username};
use crate::schema::posts::dsl::posts;
use crate::schema::posts::columns::created_at;
use crate::auth::{hash_password, verify_password};
use actix_multipart::Multipart;
use futures_util::StreamExt;
use std::io::Write;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[get("/")]
pub async fn index(tera: web::Data<Tera>, session: Session) -> impl Responder {
    let mut context = tera::Context::new();
    if let Some(user_name) = session.get::<String>("username").unwrap() {
        context.insert("username", &user_name);
    }
    HttpResponse::Ok()
        .content_type("text/html")
        .body(tera.render("index.html", &context).unwrap())
}

#[get("/education")]
pub async fn education(tera: web::Data<Tera>, session: Session) -> impl Responder {
    let mut context = tera::Context::new();
    if let Some(user_name) = session.get::<String>("username").unwrap() {
        context.insert("username", &user_name);
    }
    HttpResponse::Ok()
        .content_type("text/html")
        .body(tera.render("education.html", &context).unwrap())
}

#[get("/screening")]
pub async fn screening(tera: web::Data<Tera>, session: Session) -> impl Responder {
    let mut context = tera::Context::new();
    if let Some(user_name) = session.get::<String>("username").unwrap() {
        context.insert("username", &user_name);
    }
    HttpResponse::Ok()
        .content_type("text/html")
        .body(tera.render("screening.html", &context).unwrap())
}

#[get("/community")]
pub async fn community(tera: web::Data<Tera>, pool: web::Data<DbPool>, session: Session) -> impl Responder {
    let conn = &mut pool.get().unwrap();
    let post_list = posts
        .order(created_at.desc())
        .load::<Post>(conn)
        .expect("Error loading posts");

    let mut context = tera::Context::new();
    context.insert("posts", &post_list);
    if let Some(user_name) = session.get::<String>("username").unwrap() {
        context.insert("username", &user_name);
    }
    HttpResponse::Ok()
        .content_type("text/html")
        .body(tera.render("community.html", &context).unwrap())
}

#[post("/community")]
pub async fn create_post(
    form: web::Form<NewPost>,
    pool: web::Data<DbPool>,
    tera: web::Data<Tera>,
    session: Session,
) -> impl Responder {
    let user_name = match session.get::<String>("username").unwrap() {
        Some(user) => user,
        None => return HttpResponse::Found().append_header(("Location", "/login")).finish(),
    };

    let conn = &mut pool.get().unwrap();
    let user = users
        .filter(username.eq(&username))
        .first::<User>(conn)
        .expect("Error loading user");

    let new_post = NewPost {
        title: form.title.clone(),
        content: form.content.clone(),
        author: user_name.clone(),
        user_id: user.id,
    };

    diesel::insert_into(posts)
        .values(&new_post)
        .execute(conn)
        .expect("Error inserting post");

    let post_list = posts
        .order(created_at.desc())
        .load::<Post>(conn)
        .expect("Error loading posts");

    let mut context = tera::Context::new();
    context.insert("posts", &post_list);
    context.insert("username", &user_name);
    context.insert("message", "帖子发布成功！");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(tera.render("community.html", &context).unwrap())
}

#[get("/resources")]
pub async fn resources(tera: web::Data<Tera>, session: Session) -> impl Responder {
    let mut context = tera::Context::new();
    if let Some(user_name) = session.get::<String>("username").unwrap() {
        context.insert("username", &user_name);
    }
    HttpResponse::Ok()
        .content_type("text/html")
        .body(tera.render("resources.html", &context).unwrap())
}

#[get("/login")]
pub async fn login_page(tera: web::Data<Tera>) -> impl Responder {
    let mut context = tera::Context::new();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(tera.render("login.html", &context).unwrap())
}

#[post("/login")]
pub async fn login(
    form: web::Form<NewUser>,
    pool: web::Data<DbPool>,
    session: Session,
    tera: web::Data<Tera>,
) -> impl Responder {
    let conn = &mut pool.get().unwrap();
    let user = users
        .filter(username.eq(&form.username))
        .first::<User>(conn)
        .optional()
        .expect("Error loading user");

    let mut context = tera::Context::new();
    match user {
        Some(u) => {
            if verify_password(&form.password, &u.password).unwrap_or(false) {
                session.insert("username", &u.username).unwrap();
                context.insert("username", &u.username);
                context.insert("message", "登录成功！");
                return HttpResponse::Ok()
                    .content_type("text/html")
                    .body(tera.render("index.html", &context).unwrap());
            }
        }
        None => {}
    }
    context.insert("error", "用户名或密码错误！");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(tera.render("login.html", &context).unwrap())
}

#[get("/register")]
pub async fn register_page(tera: web::Data<Tera>) -> impl Responder {
    let mut context = tera::Context::new();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(tera.render("register.html", &context).unwrap())
}

#[post("/register")]
pub async fn register(
    form: web::Form<NewUser>,
    pool: web::Data<DbPool>,
    tera: web::Data<Tera>,
) -> impl Responder {
    let conn = &mut pool.get().unwrap();
    let hashed_password = hash_password(&form.password).unwrap();
    let new_user = NewUser {
        username: form.username.clone(),
        password: hashed_password,
    };

    let mut context = tera::Context::new();
    match diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
    {
        Ok(_) => {
            context.insert("message", "注册成功，请登录！");
            HttpResponse::Ok()
                .content_type("text/html")
                .body(tera.render("login.html", &context).unwrap())
        }
        Err(_) => {
            context.insert("error", "用户名已存在！");
            HttpResponse::Ok()
                .content_type("text/html")
                .body(tera.render("register.html", &context).unwrap())
        }
    }
}

#[get("/logout")]
pub async fn logout(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Found().append_header(("Location", "/")).finish()
}

#[post("/upload_video")]
pub async fn upload_video(mut payload: Multipart, session: Session) -> impl Responder {
    let user_name = match session.get::<String>("username").unwrap() {
        Some(user) => user,
        None => return HttpResponse::Found().append_header(("Location", "/login")).finish(),
    };

    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        let filename = field.content_disposition()
            .and_then(|cd| cd.get_filename())
            .unwrap_or("video.mp4");
        let filepath = format!("uploads/{}", filename);
        let mut f = web::block(move || std::fs::File::create(&filepath)).await.unwrap().unwrap();
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f = web::block(move || f.write_all(&data).map(|_| f)).await.unwrap().unwrap();
        }
    }
    HttpResponse::Ok().body("视频上传成功！")
}