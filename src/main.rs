#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(use_extern_macros)]
#![feature(custom_derive)]

extern crate posts;
#[macro_use]
extern crate clap;
extern crate diesel;
extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;

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
	build_rocket(
		args.value_of("ip_address").unwrap(),
		args.value_of("port").unwrap().parse::<u16>().unwrap(),
	).launch();
}

fn database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set")
}

// TODO: move this to web.rs
use rocket::Rocket;
use rocket::config::{Config, Environment};
use rocket_contrib::Json;
use rocket::http::RawStr;
use rocket::request::FromFormValue;
use self::models::Post;

struct SingletonParam;

#[derive(FromForm)]
struct Show {
	all: Option<SingletonParam>,
}

impl<'v> FromFormValue<'v> for SingletonParam {
	type Error = ();

	fn from_form_value(_ : &'v RawStr) -> Result<SingletonParam, ()> {
		Ok(SingletonParam {})
	}
}

#[get("/?<all>")]
fn index(all: Show) -> Json<Vec<Post>> {
	let show_all = match all.all {
		Some(_) => true,
		None => false,
	};
	let posts = top_posts(&establish_connection(database_url()), show_all);
	Json(posts)
}

pub fn build_rocket(address: &str, port: u16) -> Rocket {
	let config = Config::build(Environment::active().unwrap())
		.address(address)
		.port(port)
		.finalize()
		.unwrap();
	let rocket_instance = rocket::custom(config, false);
	rocket_instance.mount("/", routes![index])
}
