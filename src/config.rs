use crate::flags::{feed_arg, sqlite_file_arg};
use clap::{Arg, ArgAction, Command};

#[derive(Clone, Debug)]
pub struct Config {
    pub feed_url: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_email: String,
    pub smtp_pass: String,
    pub sqlite_file: String,
    pub port: u16,
    pub action: String,
    pub subscribe_html: Option<String>,
    pub unsubscribe_html: Option<String>,
    pub send_latest_if_skipped: bool,
}

pub fn get_config() -> Config {
    let matches = Command::new("rss-newsletter")
        .version("1.0")
        .author("Your Name <your_email@example.com>")
        .about("Sends newsletter updates from an RSS feed")
        .arg(feed_arg())
        .arg(sqlite_file_arg())
        .arg(
            Arg::new("smtp-host")
                .long("smtp-host")
                .value_name("HOSTNAME")
                .help("Hostname of the SMTP server")
                .default_value("smtp.gmail.com")
                .env("SMTP_HOST"),
        )
        .arg(
            Arg::new("smtp-port")
                .long("smtp-port")
                .value_name("INT")
                .help("Port for the SMTP server")
                .default_value("587")
                .env("SMTP_PORT"),
        )
        .arg(
            Arg::new("smtp-email")
                .long("smtp-email")
                .value_name("EMAIL")
                .help("Email address to send email from")
                .action(ArgAction::Set)
                .env("SMTP_EMAIL"),
        )
        .arg(
            Arg::new("smtp-pass")
                .long("smtp-pass")
                .value_name("STRING")
                .help("Password for the SMTP email address")
                .action(ArgAction::Set)
                .env("SMTP_PASS"),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .value_name("INT")
                .help("A port for the server")
                .default_value("80")
                .env("PORT"),
        )
        .arg(
            Arg::new("action")
                .long("action")
                .value_name("serve|blast")
                .help("Action to perform: `serve` or `blast`")
                .value_parser(clap::builder::PossibleValuesParser::new(["serve", "blast"]))
                .env("ACTION"),
        )
        // .arg(
        //     Arg::new("subscribe-html")
        //         .long("subscribe-html")
        //         .value_name("PATH")
        //         .help("Path to a file for an HTML template for subscription")
        //         .action(ArgAction::Set),
        // )
        // .arg(
        //     Arg::new("unsubscribe-html")
        //         .long("unsubscribe-html")
        //         .value_name("PATH")
        //         .help("Path to a file for an HTML template for unsubscribe")
        //         .action(ArgAction::Set),
        // )
        // .arg(
        //     Arg::new("send-latest-if-skipped")
        //         .long("send-latest-if-skipped")
        //         .value_name("BOOLEAN")
        //         .help("Send the latest email even if it's been skipped")
        //         .action(ArgAction::SetTrue),
        // )
        .arg_required_else_help(true)
        .get_matches();

    Config {
        feed_url: matches
            .get_one::<String>("feed")
            .expect("Missing feed URL")
            .clone(),
        smtp_host: matches
            .get_one::<String>("smtp-host")
            .expect("Missing SMTP host")
            .clone(),
        smtp_port: matches
            .get_one::<String>("smtp-port")
            .expect("Missing SMTP port")
            .parse()
            .expect("Invalid SMTP port"),
        smtp_email: matches
            .get_one::<String>("smtp-email")
            .expect("Missing SMTP email")
            .clone(),
        smtp_pass: matches
            .get_one::<String>("smtp-pass")
            .expect("Missing SMTP password")
            .clone(),
        sqlite_file: matches
            .get_one::<String>("sqlite-file")
            .expect("Missing SQLite file path")
            .clone(),
        port: matches
            .get_one::<String>("port")
            .expect("Missing port")
            .parse()
            .expect("Invalid port"),
        action: matches
            .get_one::<String>("action")
            .expect("Missing action")
            .clone(),
        subscribe_html: matches
            .get_one::<String>("subscribe-html")
            .map(|s| s.clone()),
        unsubscribe_html: matches
            .get_one::<String>("unsubscribe-html")
            .map(|s| s.clone()),
        send_latest_if_skipped: matches.get_flag("send-latest-if-skipped"),
    }
}
