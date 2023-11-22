# rocket+diesel+mysql项目整合
整个项目都是以最新框架版本进行整合，踩坑无数次，网上的教程都是残缺不全，要么版本老旧，这点必须吐槽rust生态是很烂，框架文档也是稀烂，很多问题都是看源码解决的。希望本教程能给刚学习rust的朋友一些帮助。

开发环境：win11+wsl2，rust版本`1.76.0-nightly`，rocket版本`0.5.0`，diesel版本 `2.1.0`，mysql版本`8.0`。

## 第一步，安装rust环境
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
## 设置rust为nightly或者dev都行，不要stable。
rustup default nightly
```
#### 💡踩个小坑
第一个坑在这里，如果不把rust设置为dev或者nightly后面安装diesel会报错，别问为啥报错，问就是框架就这样。


## 第二步，安装diesel_cli
```shell
cargo install diesel_cli --no-default-features --features mysql
```
#### 💡踩个小坑
如果不出意外，这里一定会报错，因为这个库底层依赖`mysqlclient`，更令人意外的是这个库是`python`的，所以你必须要在wsl2里安装好python环境，建议python版本3.10左右。
```
note: ld: library not found for -lmysqlclient
clang: error: linker command failed with exit code 1 (use -v to see invocation)
```

下面先安装`mysqlclient`。
```shell
## 安装环境依赖
sudo apt install default-libmysqlclient-dev build-essential
pip install mysqlclient
```

## 第三步，初始化工程
#### 初始化项目
```shell
cargo new --lib rust-blog

cd rust-blog
```
#### 修改Cargo.toml的依赖：
```
[dependencies]
rocket = {version = "0.5.0", features =["json"]}
diesel = { version = "2.1.0", features = ["mysql", "r2d2", "chrono"] }
r2d2 = "0.8.10"
r2d2_mysql = "23.0.0"
rocket_sync_db_pools = { version = "0.1.0", features = ["diesel", "diesel_mysql_pool"] }
serde = { version = "1.0", features = ["derive"] }
# Powerful date and time functionality
chrono = { version = "0.4.15", features = ["serde"] }
```
#### 创建数据库配置
创建`.env`文件, 里面是你的mysql数据库地址，
```
DATABASE_URL=mysql://devbox:mypassword@localhost/my_blog
```
创建`diesel.toml`配置文件
```
# For documentation on how to configure this file,
# see https://diesel.rs/guides/configuring-diesel-cli

[print_schema]
file = "src/schema.rs"
custom_type_derives = ["diesel::query_builder::QueryId"]

[migrations_directory]
dir = "migrations"

```
#### 执行diesel命令，生成代码
```
diesel migration generate create_users
```
修改mirations目录下的up.sql和down.sql。
```sql
---- up.sql start-------
CREATE TABLE blog_users (
    id BIGINT AUTO_INCREMENT PRIMARY KEY COMMENT '用户唯一标识',
    username VARCHAR(50) NOT NULL COMMENT '用户名',
    password_hash VARCHAR(255) NOT NULL COMMENT '存储加密后的密码',
    email VARCHAR(100) COMMENT '用户电子邮件地址',
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '记录创建时间'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='存储用户信息';
---- up.sql end-------

---- down.sql start-------
DROP TABLE blog_users;
---- up.sql end-------

```

执行`diesel migration run`生成schema.rs文件。
执行`diesel migration redo`测试down.sql是否生效。


#### 创建main.rs，跑一下hello,world
```rust
// main.rs
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
```
执行`cargo build`，`cargo run`，访问`localhost:8000`，检验一下项目。

## 第四步、创建项目结构，
整体的项目结构如下，前期为了项目入手难度低，所有的模块都在src根目录，这样比较方便简单。而且所有的mod定义也都在main.rs里，这样各个文件引用也简单。

```shell
.
├── Cargo.toml
├── README.md
├── Rocket.toml         ### rocket框架配置
├── diesel.toml         ### 数据库链接配置
└── src
    ├── db_conn.rs      ### 数据库链接配置
    ├── main.rs         ### 服务启动文件
    ├── models.rs       ### 全局model的定义
    ├── routes.rs       ### 路由文件
    ├── schema.rs       ### diesel生成的文件
    └── user_lib.rs     ### service核心逻辑
├── migrations
│   └── 2023-11-20-123055_create_users
│       ├── down.sql
│       └── up.sql
```
## 第五步，创建数据库连接
#### 修改`db_conn.rs`，
```rust
use rocket_sync_db_pools::{database, diesel};
// 数据库连接
#[database("mysql_db")]
pub struct DbConn(diesel::MysqlConnection);
```
#### 💡踩个小坑
这里有个坑，刚开始`diesel`就是无法引入进来，最后在源码里找到了答案。也就是依赖里`feature`必须要有以下三个之一，才会有`diesel`
```rust
#[cfg(any(
    feature = "diesel_sqlite_pool",
    feature = "diesel_postgres_pool",
    feature = "diesel_mysql_pool"
))]
pub use diesel;
```
#### 然后修改main.rs，把数据库相关加进去
```rust
mod db_conn;
use db_conn::DbConn;

fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", get_routes())
}
```


## 第六步，修改models文件
这一步会把crud需要的对象创建好。
```rust
use serde::{Serialize, Deserialize};
use crate::schema::blog_users;
use diesel::prelude::*;

// 对应于 blog_users 表的 Rust 结构体
#[derive(Serialize, Deserialize, Queryable, Identifiable, AsChangeset, Clone)]
#[diesel(table_name = blog_users)]
pub struct BlogUser {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub email: Option<String>,
    pub create_time: Option<chrono::NaiveDateTime>,
}

// 用于创建新用户的结构体，不包含 id 和 create_time 字段
#[derive(Serialize, Deserialize, Insertable, Clone)]
#[diesel(table_name = blog_users)]
pub struct NewBlogUser {
    pub username: String,
    pub password_hash: String,
    pub email: Option<String>,
}
```

## 第七步，修改use_lib文件
user_lib可以看成是service文件，crud核心逻辑都在这里。
```rust
use diesel::prelude::*;
use crate::models::{BlogUser, NewBlogUser};
use crate::db_conn::DbConn;

pub async fn create_user(conn: &DbConn, new_user: NewBlogUser) -> QueryResult<usize> {
    use crate::schema::blog_users::dsl::*;
    let new_user_clone = new_user.clone(); // 克隆 new_user
    conn.run(move |c| {
        diesel::insert_into(blog_users)
            .values(&new_user_clone) // 使用克隆
            .execute(c)
    }).await
}

pub async fn get_user(conn: &DbConn, user_id: i64) -> QueryResult<BlogUser> {
    use crate::schema::blog_users::dsl::*;

    conn.run(move |c| {
        blog_users.find(user_id).first::<BlogUser>(c)
    }).await
}

pub async fn update_user(conn: &DbConn, user_id: i64, user_data: BlogUser) -> QueryResult<usize> {
    use crate::schema::blog_users::dsl::*;
    let new_user_clone = user_data.clone(); // 克隆 new_user

    conn.run(move |c| {
        diesel::update(blog_users.find(user_id))
            .set(&new_user_clone)
            .execute(c)
    }).await
}

pub async fn delete_user(conn: &DbConn, user_id: i64) -> QueryResult<usize> {
    use crate::schema::blog_users::dsl::*;

    conn.run(move |c| {
        diesel::delete(blog_users.find(user_id))
            .execute(c)
    }).await
}

```
#### 💡踩个小坑
这里有个坑，就是models的对象，和schema里的对象必须完全一致。否则在查询的时候会出现类型转换错误，这里的原因是models的对象个别字段没加`Option`,如果schema里字段有`Nullable`，models的对象`Option`必须要加上。
```
 the trait bound `(BigInt, Text, Text, diesel::sql_types::Nullable<Text>, diesel::sql_types::Nullable<diesel::sql_types::Timestamp>): load_dsl::private::CompatibleType<BlogUser, Mysql>` is not satisfied
    --> src/user_lib.rs:19:52
```

## 第八步，修改routes文件
这里一次性把所有的route都创建好，统一放到routes.rs文件，然后在main.rs里引用routes，进行路由。
```rust
use rocket::serde::json::Json;
use rocket::http::Status;
use crate::db_conn::DbConn;
use crate::models::{BlogUser, NewBlogUser};
use crate::user_lib as lib;  // 引入 lib.rs 中的函数

#[get("/")]
pub fn index() -> &'static str {
    "Welcome to the Blog API"
}

#[post("/users/create", data = "<user>")]
pub async fn create_user(conn: DbConn, user: Json<NewBlogUser>) -> Status {
    match lib::create_user(&conn, user.into_inner()).await {
        Ok(_) => Status::Created,
        Err(_) => Status::InternalServerError,
    }
}

#[get("/users/<id>")]
pub async fn get_user(conn: DbConn, id: i64) -> Result<Json<BlogUser>, Status> {
    lib::get_user(&conn, id).await
        .map(Json)
        .map_err(|_| Status::NotFound)
}

#[put("/users/<id>", data = "<user>")]
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
    routes![
        index,
        create_user,
        get_user,
        update_user,
        delete_user
    ]
}

```
#### 💡踩个小坑
`rocket`配置依赖的时候，也得设置`feature`，要不然json找不到。

## 第九步，统一请求返回结构
先定义通用的返回结构：
```json
{
    "code": 200,
    "message": "ok",
    "data": {

    }
}
```
#### 修改models.rs
先定义一个通用返回类型`ResData`
```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct ResData<T>{
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}
```
#### 修改routes.rs
以create_user为例子，进行返回值的修改。其他接口的返回值与其类似。
```rust
#[post("/users/create", data = "<user>", format = "application/json")]
pub async fn create_user(conn: DbConn, user: Json<NewBlogUser>) -> Json<ResData<String>> {
    match lib::create_user(&conn, user.into_inner()).await {
        Ok(_) => Json(ResData{code:0, message: String::from("ok"), data: None }),
        Err(_) => Json(ResData{code:500, message: String::from("ok"), data: None }),
    }
}

```

## 第十步，修改最终的main.rs
```rust
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

```

#### 修改配置，调试代码
修改Rocket.tom文件
```
[global]
port = 9900

[global.databases]
mysql_db = { url = "mysql://devbox:mypassword@localhost/my_blog" }
```

执行`cargo build`,`cargo run`看看是否有编译错误，有的话根据报错进行修复。访问localhost:9900/

#### 看看成果
```
GET /users/1 text/html:
   >> Matched: (get_user) GET /users/<id>
   >> Outcome: Success(200 OK)
   >> Response succeeded.
GET / text/html:
   >> Matched: (index) GET /
   >> Outcome: Success(200 OK)
   >> Response succeeded.
```



## todo
1. 单元测试
2. 登录校验
3. 日志配置

## 参考文档
1. rust文档：https://doc.rust-lang.org/book/
2. rocket文档：
    - rocket官方文档，https://rocket.rs/v0.5/guide/getting-started/#hello-world
3. diesel文档：
    - 官方入门文档，https://diesel.rs/guides/getting-started
    - rust-doc文档，https://docs.rs/diesel/2.1.0/diesel/index.html
4. mysqlclient：https://pypi.org/project/mysqlclient/
