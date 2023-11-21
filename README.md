# rocket+diesel+mysql项目整合
整个项目都是以最新框架版本进行整合，踩坑无数次，网上的教程都是残缺不全，要么版本老旧，这点必须吐槽rust生态是很烂，框架文档也是稀烂，很多问题都是看源码解决的。希望本教程能给刚学习rust的朋友一些帮助。

开发环境：win11+wsl2，rust版本`1.76.0-nightly`，rocket版本`0.5.0`，diesel版本 `2.1.0`，mysql版本`8.0`。

## 第一步，安装rust环境
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
## 设置rust为nightly或者dev都行，不要stable。
rustup default nightly
```
第一个坑在这里，如果不把rust设置为dev或者nightly后面安装diesel会报错，别问为啥报错，问就是框架就这样。


## 第二步，安装diesel_cli
```shell
cargo install diesel_cli --no-default-features --features mysql
```
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
#### 踩个小坑
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


## 参考文档
1. rocket：
    - rocket官方文档，https://rocket.rs/v0.5/guide/getting-started/#hello-world
2. diesel：
    - 官方入门文档，https://diesel.rs/guides/getting-started
    - rust-doc文档，https://docs.rs/diesel/2.1.0/diesel/index.html
3. mysqlclient：https://pypi.org/project/mysqlclient/
