extern crate posts;
#[macro_use]
extern crate clap;
extern crate diesel;
extern crate dotenv;

mod cli;

use self::posts::*;
use dotenv::dotenv;
use std::env;

fn main() {
    match cli::build_cli()
        .get_matches()
        .subcommand() {
            ("show", Some(args)) => run_show_command(args),
            ("create", Some(_)) => run_create_command(),
            ("publish", Some(args)) => run_publish_command(args),
            ("server", Some(args)) => run_server_command(args),
            _ => unreachable!("The cli parser prevents reaching here"),
        }
}


fn run_show_command(args: &clap::ArgMatches) {
    let connection = establish_connection(database_url());
    let include_unpublished = args.is_present("all");
    let posts = top_posts(&connection, include_unpublished);
    cli::print_posts(posts);
}

fn run_publish_command(args: &clap::ArgMatches) {
    use self::diesel::prelude::*;
    use self::posts::models::Post;

    use posts::schema::posts::dsl::{posts, published};

    let id =  value_t!(args, "ID", i32).unwrap(); // clap guarantees value will have been provided.
    let connection = establish_connection(database_url());

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));

    println!("Published post: '{}'", post.title);
}

fn run_create_command() {
    let connection = establish_connection(database_url());
    let title = cli::read_title();
    let body = cli::read_body(&title);

    let post = create_post(&connection, &title, &body);
    println!("\nSaved draft {} with id {}", title, post.id);
}

fn run_server_command(args: &clap::ArgMatches) {
}

fn database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set")
}
