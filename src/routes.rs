use crate::db_conn::DbConn;
use crate::models::{BlogUser, NewBlogUser, ResData};
use crate::user_lib as lib;
use rocket::serde::json::Json; // 引入 lib.rs 中的函数
use rocket::http::{CookieJar, Status};
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::{Responder, Response};
use std::io::Cursor;


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

struct LoggedInGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for LoggedInGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let cookies = request.guard::<&CookieJar>().await.unwrap();
        if cookies.get("user_id").is_some() {
            Outcome::Success(LoggedInGuard)
        } else {
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}

struct PleaseLogin;

impl<'r, 'o: 'r> Responder<'r, 'o> for PleaseLogin {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'o> {
        let tips = "请先登录";
        Response::build()
            .status(Status::Unauthorized)
            .sized_body(tips.len(), Cursor::new(tips))
            .ok()
    }
}
