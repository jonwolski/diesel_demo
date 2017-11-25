#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use self::models::{Post, NewPost};

pub fn establish_connection(database_url: String) -> PgConnection {
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

pub fn top_posts(conn: &PgConnection, include_unpublished: bool) -> Vec<Post> {
    use schema::posts::dsl::*;

    let query = posts.limit(5).order(id);
    let result = match include_unpublished {
        true  => query.load::<Post>(conn),
        false => query.filter(published.eq(true)).load::<Post>(conn)
    };

    result.expect("Error loading posts")
}

pub fn publish_post(conn: &PgConnection, post_id: i32) -> Post {
    use schema::posts::dsl::*;

    diesel::update(posts.find(post_id))
        .set(published.eq(true))
        .get_result::<Post>(conn)
        .unwrap()
}

