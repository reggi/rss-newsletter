use clap::{Arg, ArgAction};

pub fn feed_arg() -> Arg {
    Arg::new("feed")
        .long("feed")
        .value_name("URL")
        .help("The file name of the SQLite database")
        .required(true)
        .env("FEED")
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
