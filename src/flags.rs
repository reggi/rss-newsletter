use clap::{Arg, ArgAction};

pub fn email_arg() -> Arg {
    Arg::new("email")
        .long("email")
        .short('e')
        .value_name("EMAIL")
        .help("The email address to interact with")
        .action(ArgAction::Set)
        .required(true)
}

pub fn unsubscribe_arg() -> Arg {
    Arg::new("unsubscribe")
        .long("unsubscribe")
        .short('u') // Add this line
        .value_name("BOOL")
        .help("Unsubscribe the user from the feed")
        .action(ArgAction::SetTrue)
}

pub fn feed_url_arg() -> Arg {
    Arg::new("feed-url")
        .long("feed-url")
        .short('f')
        .value_name("URL")
        .help("The url of the rss feed")
        .required(true)
        .env("FEED_URL")
        .action(ArgAction::Set)
}

pub fn sqlite_file_arg() -> Arg {
    Arg::new("sqlite-file")
        .long("sqlite-file")
        .value_name("FILE_PATH")
        .help("The file name of the SQLite database")
        .required(true)
        .env("SQLITE_FILE")
        .action(ArgAction::Set)
}

pub fn smtp_host_arg() -> Arg {
    Arg::new("smtp-host")
        .long("smtp-host")
        .value_name("HOSTNAME")
        .help("Hostname of the SMTP server")
        .default_value("smtp.gmail.com")
        .action(ArgAction::Set)
        .env("SMTP_HOST")
}

pub fn smtp_port_arg() -> Arg {
    Arg::new("smtp-port")
        .long("smtp-port")
        .value_name("INT")
        .help("Port for the SMTP server")
        .default_value("587")
        .env("SMTP_PORT")
}

pub fn smtp_email_arg() -> Arg {
    Arg::new("smtp-email")
        .long("smtp-email")
        .value_name("EMAIL")
        .help("Email address to send email from")
        .action(ArgAction::Set)
        .env("SMTP_EMAIL")
}

pub fn smtp_pass_arg() -> Arg {
    Arg::new("smtp-pass")
        .long("smtp-pass")
        .value_name("STRING")
        .help("Password for the SMTP email address")
        .action(ArgAction::Set)
        .env("SMTP_PASS")
}

pub fn port_arg() -> Arg {
    Arg::new("port")
        .long("port")
        .value_name("INT")
        .help("A port for the server")
        .default_value("80")
        .env("PORT")
}

use clap::ArgMatches;

pub struct ConfigExtractor {
    matches: ArgMatches,
}

impl ConfigExtractor {
    pub fn new(matches: ArgMatches) -> Self {
        Self { matches }
    }

    pub fn get_feed_url(&self) -> String {
        self.matches
            .get_one::<String>("feed-url")
            .expect("Missing feed URL")
            .clone()
    }

    pub fn get_smtp_host(&self) -> String {
        self.matches
            .get_one::<String>("smtp-host")
            .expect("Missing SMTP host")
            .clone()
    }

    pub fn get_smtp_port(&self) -> u16 {
        self.matches
            .get_one::<String>("smtp-port")
            .expect("Missing SMTP port")
            .parse()
            .expect("Invalid SMTP port")
    }

    pub fn get_smtp_email(&self) -> String {
        self.matches
            .get_one::<String>("smtp-email")
            .expect("Missing SMTP email")
            .clone()
    }

    pub fn get_smtp_pass(&self) -> String {
        self.matches
            .get_one::<String>("smtp-pass")
            .expect("Missing SMTP password")
            .clone()
    }

    pub fn get_sqlite_file(&self) -> String {
        self.matches
            .get_one::<String>("sqlite-file")
            .expect("Missing SQLite file path")
            .clone()
    }

    pub fn get_port(&self) -> u16 {
        self.matches
            .get_one::<String>("port")
            .expect("Missing port")
            .parse()
            .expect("Invalid port")
    }

    pub fn get_email(&self) -> String {
        self.matches
            .get_one::<String>("email")
            .expect("Missing email")
            .clone()
    }

    pub fn get_unsubscribe(&self) -> bool {
        self.matches.get_flag("unsubscribe")
    }

    // pub fn get_subscribe_html(&self) -> Option<String> {
    //     self.matches.get_one::<String>("subscribe-html").map(Clone::clone)
    // }

    // pub fn get_unsubscribe_html(&self) -> Option<String> {
    //     self.matches.get_one::<String>("unsubscribe-html").map(Clone::clone)
    // }

    // pub fn get_send_latest_if_skipped(&self) -> bool {
    //     self.matches.get_flag("send-latest-if-skipped")
    // }
}
