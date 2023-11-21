use rocket_sync_db_pools::{database, diesel};
// use diesel::r2d2::{ConnectionManager, Pool};
// use diesel::MysqlConnection;

// 数据库连接池
#[database("mysql_db")]
pub struct DbConn(diesel::MysqlConnection);

