#[macro_use] extern crate rocket;
extern crate diesel;
mod schema;
mod models;
mod routes;
mod db_conn;
mod user_lib;
use routes::get_routes;
use db_conn::DbConn;


// Rocket 启动函数
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", get_routes())
}
