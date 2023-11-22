# rust-blog第二个版本
这个版本是在v1的基础上进行改进，v1版本代码开源从GitHub的tag下载，https://github.com/maochunguang/rust-blog/releases/tag/v1.0，新版本v2增加以下功能：
1. 登录，登出，注册功能
2. 登录态校验，
3. 把登录校验做出通用的宏模块
    1. `#[auth_login(param = "user_id", method = "login")]`



## 第一步，修改路由
1、把user相关路由分离
2、增加login相关路由到`route_login.rs`。

## 第二步，增加登录，注册，登出等功能
