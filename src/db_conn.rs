use rocket_sync_db_pools::{database, diesel};

// 数据库连接池
#[database("mysql_db")]
pub struct DbConn(diesel::MysqlConnection);

