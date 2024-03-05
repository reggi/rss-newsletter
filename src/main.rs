mod blast;
mod db;
mod email_sender;
mod flags;
mod model;
mod routes;
mod rss_processor;
mod server;
mod types;
mod utils;

use blast::blast;
use model::Model;
use server::main as server_main;
use types::{BlastConfig, BlastContext, ServerConfig, ServerContext, SubscriberConfig};
use email_sender::{EmailConfig, send_email};

use flags::{
    email_arg, feed_url_arg, port_arg, smtp_email_arg, smtp_host_arg, smtp_pass_arg, smtp_port_arg,
    sqlite_file_arg, unsubscribe_arg, ConfigExtractor,
};

use clap::Command;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let matches = Command::new("rss-newsletter")
        .version("1.0")
        .about("Manages subscriptions and sends updates")
        .subcommand(
            Command::new("blast")
                .about("Sends the latest RSS post to all subscribers")
                .arg(sqlite_file_arg())
                .arg(smtp_host_arg())
                .arg(smtp_port_arg())
                .arg(smtp_email_arg())
                .arg(smtp_pass_arg())
                .arg(feed_url_arg())
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("server")
                .alias("serve")
                .about("Starts the server for subscribing/unsubscribing to the newsletter")
                .arg(sqlite_file_arg())
                .arg(port_arg())
                .arg(feed_url_arg())
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("subscriber")
                .alias("sub")
                .alias("subsrcibe")
                .about("Interact with a subscriber.")
                .arg(feed_url_arg())
                .arg(sqlite_file_arg())
                .arg(unsubscribe_arg())
                .arg(email_arg()),
        )
        .subcommand(
            Command::new("test-email")
                .about("Send a test email to the provided email address.")
                .arg(smtp_host_arg())
                .arg(smtp_port_arg())
                .arg(smtp_email_arg())
                .arg(smtp_pass_arg())
                .arg(email_arg()),
        )
        .arg_required_else_help(true)
        .get_matches();

    match matches.subcommand() {
        Some(("blast", sub_m)) => {
            let ce = ConfigExtractor::new(sub_m.clone());

            let config = BlastConfig {
                sqlite_file: ce.get_sqlite_file(),
                smtp_host: ce.get_smtp_host(),
                smtp_port: ce.get_smtp_port(),
                smtp_email: ce.get_smtp_email(),
                smtp_pass: ce.get_smtp_pass(),
                feed_url: ce.get_feed_url(),
            };

            let model = Model::new(&config.sqlite_file, &config.feed_url)
                .await
                .expect("Failed to create model");

            let context = BlastContext { config, model };

            blast(context).await?;

            Ok(())
        }
        Some(("server", sub_m)) => {
            let ce = ConfigExtractor::new(sub_m.clone());

            let config = ServerConfig {
                sqlite_file: ce.get_sqlite_file(),
                feed_url: ce.get_feed_url(),
                port: ce.get_port(),
            };

            let model = Model::new(&config.sqlite_file, &config.feed_url)
                .await
                .expect("Failed to create model");

            let context = ServerContext { model, config };

            server_main(context).await?;

            Ok(())
        }
        Some(("test-email", sub_m)) => {
            
            let ce: ConfigExtractor = ConfigExtractor::new(sub_m.clone());

            let config = EmailConfig {
                body: "This is a test email".to_string(),
                subject: "Test Email".to_string(),
                smtp_password: ce.get_smtp_pass(),
                smtp_host: ce.get_smtp_host(),
                smtp_port: ce.get_smtp_port(),
                smtp_email: ce.get_smtp_email(),
                to: ce.get_email(),
            };

            println!("Sending email to: {}", config);

            send_email(config).expect("Failed to send email");

            Ok(())
        }
        Some(("subscriber", sub_m)) => {
            let ce: ConfigExtractor = ConfigExtractor::new(sub_m.clone());

            let config = SubscriberConfig {
                sqlite_file: ce.get_sqlite_file(),
                feed_url: ce.get_feed_url(),
                email: ce.get_email(),
                unsubscribe: ce.get_unsubscribe(),
            };

            let model = Model::new(&config.sqlite_file, &config.feed_url)
                .await
                .expect("Failed to create model");
            if config.unsubscribe {
                model
                    .unsubscribe(&config.email)
                    .await
                    .expect("Failed to unsubscribe");
            } else {
                model
                    .subscribe(&config.email)
                    .await
                    .expect("Failed to Subscribe");
            }

            let sub = model
                .get_subscriber(&config.email)
                .await
                .expect("Failed to get subscriber");

            match sub {
                Some(sub) => print!("Subscriber: {}", sub),
                None => print!("Subscriber not found"),
            }

            Ok(())
        }
        _ => unreachable!("The CLI parser ensures that only defined subcommands are used"),
    }
}
