use crate::db_conn::DbConn;
use crate::models::{BlogUser, LoginUser, NewBlogUser, ResData};
use crate::user_lib as lib;
use rocket::http::{CookieJar, Cookie};
use rocket::serde::json::Json; // 引入 lib.rs 中的函数
use chrono::Local;



#[post("/login", data = "<user>", format = "application/json")]
pub async fn auth_login(conn: DbConn, user: Json<LoginUser>, cookies: &CookieJar<'_>,) -> Json<ResData<BlogUser>> {
    match lib::get_login_user(&conn, user.into_inner()).await {
        Ok(_) => {
            // 如果登录成功，设置带有上次活动时间戳的 cookie
            cookies.add(
                Cookie::build(("user_id", Local::now().timestamp().to_string())),
            );
            Json(ResData {
                code: 0,
                message: String::from("login success"),
                data: None,
            })
        }

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
#[post("/regiter", data = "<user>", format = "application/json")]
pub async fn auth_regiter(conn: DbConn, user: Json<NewBlogUser>) -> Json<ResData<String>> {
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
    routes![auth_login, auth_logout, auth_regiter]
}
