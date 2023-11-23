use rocket::request::Outcome;
use rocket::{Request, request};
use rocket::http::{Cookie, Status, ContentType};
use rocket::response::{Responder, Response};
use rocket::serde::json::Json; // 引入 lib.rs 中的函数
use std::io::Cursor;
use chrono::{Duration, Local};
use crate::models::ResData;
use serde_json;

pub struct LoggedInGuard;

#[rocket::async_trait]
impl<'r> request::FromRequest<'r> for LoggedInGuard {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let cookies = request.cookies();

        if let Some(cookie) = cookies.get("user_id") {
            let last_activity = cookie
                .value()
                .parse::<i64>()
                .unwrap();
    
            if Local::now().timestamp() - last_activity < Duration::hours(1).num_seconds() {
                cookies.add(
                    Cookie::build(("user_id", Local::now().timestamp().to_string())),
                );
                Outcome::Success(LoggedInGuard)
            } else {
                cookies.remove("user_id"); // 清除过时的 cookie
                Outcome::Error((Status::Unauthorized, "请登录".to_string()))
            }
        } else {
            Outcome::Error((Status::Unauthorized, "请登录".to_string()))
        }
    }
}
pub struct PleaseLogin;
impl<'r, 'o: 'r> Responder<'r, 'o> for PleaseLogin {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'o> {
        let res_data :ResData<String> = ResData {
            code: 0,
            message: String::from("ok"),
            data: None,
        };
        let tips =  serde_json::to_string(&res_data).unwrap();
        Response::build()
            .header(ContentType::JSON)
            .status(Status::Unauthorized)
            .sized_body(tips.len(), Cursor::new(tips))
            .ok()
    }
}

// 未登录时的响应
#[catch(401)]
pub fn not_logged_in() -> PleaseLogin {
    PleaseLogin
}


