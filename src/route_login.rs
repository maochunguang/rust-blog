use crate::db_conn::DbConn;
use crate::models::{BlogUser,LoginUser, NewBlogUser, ResData};
use crate::user_lib as lib;
use rocket::serde::json::Json; // 引入 lib.rs 中的函数
// use rocket::http::{CookieJar, Status};
// use rocket::outcome::Outcome;
// use rocket::request::{self, FromRequest, Request};
// use rocket::response::{Redirect, Responder, Response};


#[post("/login", data = "<user>", format = "application/json")]
pub async fn auth_login(conn: DbConn, user: Json<LoginUser>) -> Json<ResData<BlogUser>> {
    match lib::get_login_user(&conn, user.into_inner()).await {
        Ok(user) => Json(ResData {
            code: 0,
            message: String::from("ok"),
            data: Some(user),
        }),
        Err(e) => Json(ResData {
            code: 500,
            message: e.to_string(),
            data: None,
        }),
    }
}

#[post("/logout", data = "<user>", format = "application/json")]
pub async fn auth_logout(conn: DbConn, user: Json<NewBlogUser>) -> Json<ResData<String>> {
    match lib::create_user(&conn, user.into_inner()).await {
        Ok(_) => Json(ResData {
            code: 0,
            message: String::from("ok"),
            data: None,
        }),
        Err(e) => Json(ResData {
            code: 500,
            message: e.to_string(),
            data: None,
        }),
    }
}

pub fn get_login_routes() -> Vec<rocket::Route> {
    routes![auth_login, auth_logout]
}
