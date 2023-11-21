// @generated automatically by Diesel CLI.

// diesel::table! {
//     blog_posts (id) {
//         id -> Bigint,
//         user_id -> Bigint,
//         #[max_length = 100]
//         title -> Varchar,
//         #[max_length = 2048]
//         content -> Varchar,
//         published -> Nullable<Bool>,
//         create_time -> Nullable<Timestamp>,
//         update_time -> Nullable<Timestamp>,
//     }
// }

diesel::table! {
    blog_users (id) {
        id -> Bigint,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 100]
        email -> Nullable<Varchar>,
        create_time -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    // blog_posts,
    blog_users,
);
