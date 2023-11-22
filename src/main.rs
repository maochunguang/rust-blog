#[macro_use] extern crate rocket;
extern crate diesel;
mod schema;
mod models;
mod routes;
mod route_login;
mod db_conn;
mod user_lib;
use routes::get_routes;
use route_login::get_login_routes;
use db_conn::DbConn;


// Rocket 启动函数
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/auth", get_login_routes())
        .mount("/users", get_routes())
}
