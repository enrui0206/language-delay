// src/models.rs
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use diesel::sqlite::Sqlite;

#[derive(Queryable, Serialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Selectable)]
#[derive(Queryable, Serialize, Insertable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(Sqlite))] 
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub author: String,
    pub user_id: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub author: String,
    pub user_id: i32,
}