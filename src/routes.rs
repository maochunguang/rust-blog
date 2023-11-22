use crate::db_conn::DbConn;
use crate::models::{BlogUser, NewBlogUser, ResData};
use crate::user_lib as lib;
use rocket::serde::json::Json; // 引入 lib.rs 中的函数

#[get("/")]
pub fn index() -> Json<ResData<String>> {
    Json(ResData {
        code: 0,
        message: String::from("ok"),
        data: Some(String::from("hello world!")),
    })
}

#[post("/users/create", data = "<user>", format = "application/json")]
pub async fn create_user(conn: DbConn, user: Json<NewBlogUser>) -> Json<ResData<String>> {
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

#[get("/users/<id>")]
pub async fn get_user(conn: DbConn, id: i64) -> Json<ResData<BlogUser>> {
    match lib::get_user(&conn, id).await {
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

#[put("/users/<id>", data = "<user>", format = "application/json")]
pub async fn update_user(conn: DbConn, id: i64, user: Json<BlogUser>) -> Json<ResData<String>> {
    match lib::update_user(&conn, id, user.into_inner()).await {
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

#[delete("/users/<id>")]
pub async fn delete_user(conn: DbConn, id: i64) -> Json<ResData<String>> {
    match lib::delete_user(&conn, id).await {
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

pub fn get_routes() -> Vec<rocket::Route> {
    routes![index, create_user, get_user, update_user, delete_user]
}
