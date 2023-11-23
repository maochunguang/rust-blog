use crate::db_conn::DbConn;
use crate::models::{BlogUser, NewBlogUser, ResData};
use crate::user_lib as lib;
use crate::login_lib::LoggedInGuard;
use rocket::serde::json::Json; // 引入 lib.rs 中的函数

#[get("/")]
pub fn index() -> Json<ResData<String>> {
    Json(ResData {
        code: 0,
        message: String::from("ok"),
        data: Some(String::from("hello world!")),
    })
}

#[post("/create", data = "<user>", format = "application/json")]
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

#[get("/<id>")]
pub async fn get_user(conn: DbConn, id: i64, _logged_in: LoggedInGuard) -> Json<ResData<BlogUser>> {
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

#[put("/<id>", data = "<user>", format = "application/json")]
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

#[delete("/<id>")]
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
