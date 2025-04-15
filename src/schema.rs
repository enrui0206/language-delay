use diesel::table;
use diesel::joinable;
use diesel::allow_tables_to_appear_in_same_query;

// 此文件由 Diesel 自动生成，运行 `diesel migration run` 后生成
table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}

table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        content -> Text,
        created_at -> Timestamp,
        author -> Text,
        user_id -> Integer,
    }
}

joinable!(posts -> users (user_id));
allow_tables_to_appear_in_same_query!(users, posts);