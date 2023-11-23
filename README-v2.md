# rust-blog第二个版本
这个版本是在v1的基础上进行改进，v1版本代码开源从GitHub的tag下载，https://github.com/maochunguang/rust-blog/releases/tag/v1.0，新版本v2增加以下功能：
1. 登录，登出，注册功能
2. 登录态校验，
3. 把登录校验做出通用的宏模块
    1. `#[auth_login(param = "user_id", method = "login")]`



## 第一步，修改路由
1、把user相关路由分离
2、增加login相关路由到`route_login.rs`。
```rust
#[post("/login", data = "<user>", format = "application/json")]
pub async fn auth_login(conn: DbConn, user: Json<LoginUser>, cookies: &CookieJar<'_>,) -> Json<ResData<BlogUser>> {

}
#[post("/logout", data = "<user>", format = "application/json")]
pub async fn auth_logout(conn: DbConn, user: Json<NewBlogUser>, cookies: &CookieJar<'_>,) -> Json<ResData<String>> {

}
#[post("/regiter", data = "<user>", format = "application/json")]
pub async fn auth_regiter(conn: DbConn, user: Json<NewBlogUser>) -> Json<ResData<String>> {

}
pub fn get_login_routes() -> Vec<rocket::Route> {
    routes![auth_login, auth_logout, auth_regiter]
}
```
修改main.js把路由拆分开。
```rust
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/auth", get_login_routes())
        .mount("/users", get_routes())
        .register("/", catchers![not_logged_in])
}

```



## 第二步，增加登录，注册，登出等功能
本项目只做练习和演示，对登录和登出进行简单处理。
- 登录时，先查询用户存不存在，存在往`cookie`，写一个字段`user_id`，value是当前时间戳，1h有效期，如果下次请求`cookie`的时间超过了1h，就删除`cookie`里的字段`user_id`。
- 登出时，直接删除`cookie`里的字段`user_id`。

#### 登录功能实现
先修改`route_login.rs`，
```rust
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
```
在修改`user_lib.rs`，loginUser可以在models里自己定义。
```rust
pub async fn get_login_user(conn: &DbConn, login_user: LoginUser) -> QueryResult<BlogUser> {
    use crate::schema::blog_users::dsl::*;
    let new_user_clone = login_user.clone(); // 克隆 new_user

    conn.run(move |c| {
        blog_users.filter(username.eq(new_user_clone.username))
        .filter(password_hash.eq(new_user_clone.password_hash)).first::<BlogUser>(c)
    }).await
}
```

#### 登出功能实现
登出时，直接删除`cookie`里的字段`user_id`。
```rust
#[post("/logout", format = "application/json")]
pub async fn auth_logout(cookies: &CookieJar<'_>,) -> Json<ResData<String>> {
    cookies.remove("user_id");
    Json(ResData {
        code: 0,
        message: String::from("logout success"),
        data: None,
    }) 
}
```
## 第三步，实现请求拦截和统一未登录报错
rocket框架针对请求拦截，可以通过自定义登录守卫来实现。
#### 自定义一个登录守卫，login_lib.rs
```rust
use rocket::request::Outcome;
use rocket::{Request, request};
use rocket::http::{Cookie, Status, ContentType};
use rocket::response::{Responder, Response};
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
```
#### 统一未登录报错信息，login_lib.rs
这里的核心流程是针对所有未登录的请求，返回一个401的状态码，然后统一捕获这个状态码，返回统一的信息。
> 这里可以不止捕获401，还可以加上404的，500的等等异常提示。
```rust
// 导入的包上面都有了。
pub struct PleaseLogin;
impl<'r, 'o: 'r> Responder<'r, 'o> for PleaseLogin {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'o> {
        let res_data :ResData<String> = ResData {
            code: 0,
            message: String::from("请先登录！"),
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
```
还需要在main.js指明捕获的请求处理。
```rust
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/auth", get_login_routes())
        .mount("/users", get_routes())
        .register("/", catchers![not_logged_in])
}
```
#### 第四步，运行测试
使用cargo buld和cargo run，测试以下代码。成功如下。
```
GET /users/1:
   >> Matched: (get_user) GET /users/<id>
   >> Request guard `LoggedInGuard` failed: "请登录".
   >> Outcome: Error(401 Unauthorized)
   >> Responding with registered (not_logged_in) 401 catcher.
   >> Response succeeded.
```