#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use self::models::{Post, NewPost};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert(&new_post).into(posts::table)
        .get_result(conn)
        .expect(&format!("Saving failed for post: {}", new_post.title))
}

pub fn top_posts(conn: &PgConnection) -> Vec<Post> {
    use schema::posts::dsl::*;

    posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(conn)
        .expect("Error loading posts")
}

pub fn publish_post(conn: &PgConnection, post_id: i32) -> Post {
    use schema::posts::dsl::*;

    diesel::update(posts.find(post_id))
        .set(published.eq(true))
        .get_result::<Post>(conn)
        .unwrap()
}

