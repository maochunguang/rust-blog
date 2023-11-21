use crate::db_conn::DbConn;
use crate::models::{BlogUser, NewBlogUser};
use crate::user_lib as lib;
use rocket::http::Status;
use rocket::serde::json::Json; // 引入 lib.rs 中的函数

#[get("/")]
pub fn index() -> &'static str {
    "Welcome to the Blog API"
}

#[post("/users/create", data = "<user>", format = "application/json")]
pub async fn create_user(conn: DbConn, user: Json<NewBlogUser>) -> Status {
    match lib::create_user(&conn, user.into_inner()).await {
        Ok(_) => Status::Created,
        Err(_) => Status::InternalServerError,
    }
}

#[get("/users/<id>")]
pub async fn get_user(conn: DbConn, id: i64) -> Result<Json<BlogUser>, Status> {
    lib::get_user(&conn, id)
        .await
        .map(Json)
        .map_err(|_| Status::NotFound)
}

#[put("/users/<id>", data = "<user>", format = "application/json")]
pub async fn update_user(conn: DbConn, id: i64, user: Json<BlogUser>) -> Status {
    match lib::update_user(&conn, id, user.into_inner()).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::NotFound,
    }
}

#[delete("/users/<id>")]
pub async fn delete_user(conn: DbConn, id: i64) -> Status {
    match lib::delete_user(&conn, id).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::NotFound,
    }
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![index, create_user, get_user, update_user, delete_user]
}
