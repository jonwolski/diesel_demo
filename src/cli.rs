use clap::{App, AppSettings, Arg, SubCommand};
use std::io::{stdin,Read};
use models::Post;

pub fn build_cli() -> App<'static, 'static> {
    let show_posts_subcommand = SubCommand::with_name("show")
        .setting(AppSettings::VersionlessSubcommands)
        .about("Display all posts")
        .arg(Arg::with_name("all")
             .short("a")
             .long("all")
             .required(false)
             .help("Show all posts including unpublished in listing")
            );

    let create_posts_subcommand = SubCommand::with_name("create")
        .setting(AppSettings::VersionlessSubcommands)
        .about("Create a new post");

    let publish_posts_subcommand = SubCommand::with_name("publish")
        .setting(AppSettings::VersionlessSubcommands)
        .arg(Arg::with_name("ID")
             .help("Identifier of the post to publish. I.e. the database record's (integer) ID.")
             .index(1)
             .required(true)
             .takes_value(true)
            )
        .about("Publish a post identified by its ID");

    let server_subcommand = SubCommand::with_name("server")
        .setting(AppSettings::VersionlessSubcommands)
        .arg(Arg::with_name("port")
             .help("The Web server will listen on this port.")
             .required(false)
             .short("p")
             .long("port")
             .value_name("PORT")
             .takes_value(true)
             .default_value("8000")
            )
        .arg(Arg::with_name("ip_address")
             .help("The Web server will be bound to this IP address.")
             .required(false)
             .short("b")
             .long("binding")
             .value_name("IP_ADDRESS")
             .takes_value(true)
             .default_value("0.0.0.0")
            )
        .about("start Web server");



    App::new("dieseldemo")
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(show_posts_subcommand)
        .subcommand(create_posts_subcommand)
        .subcommand(publish_posts_subcommand)
        .subcommand(server_subcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
}

pub fn read_title() -> String {
    println!("Title:");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    title.trim().to_string()
}

pub fn read_body(title: &str) -> String {
    println!("\nOk! Let's write {} (Press {} when finished)\n", title, EOF);
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();
    body.trim().to_string()
}

pub fn print_posts(posts: Vec<Post>) {
    println!("Displaying {} posts", posts.len());
    for post in posts {
        println!("{}", post.title);
        println!("----------\n\n");
        println!("{}", post.body);
    }
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";


