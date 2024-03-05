use crate::model::Model;
use std::fmt;

pub struct BlastConfig {
    pub sqlite_file: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_email: String,
    pub smtp_pass: String,
    pub feed_url: String,
}

pub struct BlastContext {
    pub model: Model,
    pub config: BlastConfig,
}

#[derive(Clone)]
pub struct ServerConfig {
    pub sqlite_file: String,
    pub feed_url: String,
    pub port: u16,
}

impl fmt::Display for ServerConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ServerConfig {{ sqlite_file: \"{}\", feed_url: \"{}\", port: {} }}",
            self.sqlite_file, self.feed_url, self.port
        )
    }
}

#[derive(Clone)]
pub struct ServerContext {
    pub model: Model,
    pub config: ServerConfig,
}

pub struct SubscriberConfig {
    pub sqlite_file: String,
    pub feed_url: String,
    pub email: String,
    pub unsubscribe: bool,
}
