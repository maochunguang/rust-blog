use serde::{Serialize, Deserialize};
use crate::schema::blog_users;
use diesel::prelude::*;

// 对应于 blog_users 表的 Rust 结构体
#[derive(Serialize, Deserialize, Queryable, Identifiable, AsChangeset, Clone)]
#[diesel(table_name = blog_users)]
pub struct BlogUser {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub email: Option<String>,
    pub create_time: Option<chrono::NaiveDateTime>,
}

// 用于创建新用户的结构体，不包含 id 和 create_time 字段
#[derive(Serialize, Deserialize, Insertable, Clone)]
#[diesel(table_name = blog_users)]
pub struct NewBlogUser {
    pub username: String,
    pub password_hash: String,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginUser {
    pub username: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResData<T>{
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}
