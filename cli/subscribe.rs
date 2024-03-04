use clap::{Arg, ArgAction, Command};
use rssnewsletter::flags::{feed_arg, sqlite_file_arg};
use rssnewsletter::model::Model;

#[tokio::main]
async fn main() {
    print!("Starting");

    let matches = Command::new("rssnewsletter")
        .version("1.0")
        .author("Your Name <your_email@example.com>")
        .about("Sends newsletter updates from an RSS feed")
        .arg(feed_arg())
        .arg(sqlite_file_arg())
        .arg(
            Arg::new("email")
                .long("email")
                .value_name("EMAIL")
                .help("The email address to interact with")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("unsubscribe")
                .long("unsubscribe")
                .value_name("BOOL")
                .help("Unsubscribe the user from the feed")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    print!("Matches Finished");

    let feed_url = matches.get_one::<String>("feed").unwrap().clone();
    let sqlite_file = matches.get_one::<String>("sqlite-file").unwrap().clone();
    let email = matches.get_one::<String>("email").unwrap().clone();
    let unsubscribe = matches.get_flag("unsubscribe");

    match Model::new(&sqlite_file, &feed_url).await {
        Ok(model) => match unsubscribe {
            true => {
                let result = model.unsubscribe(&email).await;
                println!("Unsubscribed: {:?}", result);
            }
            false => {
                let result = model.add_subscriber(&email).await;
                println!("Added subscriber: {:?}", result);
            }
        },
        Err(e) => eprintln!("Failed to create model: {}", e),
    }
}
