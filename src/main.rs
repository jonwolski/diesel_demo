extern crate posts;
#[macro_use]
extern crate clap;
extern crate diesel;

mod cli;

use self::posts::*;

fn main() {
    match cli::build_cli()
        .get_matches()
        .subcommand() {
            ("show", Some(_)) => run_show_command(),
            ("create", Some(_)) => run_create_command(),
            ("publish", Some(args)) => run_publish_command(args),
            _ => unreachable!("The cli parser prevents reaching here"),
        }
}


fn run_show_command() {
    let connection = establish_connection();
    let results = top_posts(&connection);
    cli::print_posts(results);
}

fn run_publish_command(args: &clap::ArgMatches) {
    use self::diesel::prelude::*;
    use self::posts::models::Post;

    use posts::schema::posts::dsl::{posts, published};

    let id =  value_t!(args, "ID", i32).unwrap(); // clap guarantees value will have been provided.
    let connection = establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));

    println!("Published post: '{}'", post.title);
}

fn run_create_command() {
    let connection = establish_connection();
    let title = cli::read_title();
    let body = cli::read_body(&title);

    let post = create_post(&connection, &title, &body);
    println!("\nSaved draft {} with id {}", title, post.id);
}
