use diesel::prelude::*;
use crate::models::{BlogUser, NewBlogUser, LoginUser};
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

pub async fn get_login_user(conn: &DbConn, login_user: LoginUser) -> QueryResult<BlogUser> {
    use crate::schema::blog_users::dsl::*;
    let new_user_clone = login_user.clone(); // 克隆 new_user

    conn.run(move |c| {
        blog_users.filter(username.eq(new_user_clone.username))
        .filter(password_hash.eq(new_user_clone.password_hash)).first::<BlogUser>(c)
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
