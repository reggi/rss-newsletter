#![allow(warnings)]

mod blast;
mod config;
mod context;
mod db;
mod email_sender;
mod flags;
mod model;
mod routes;
mod rss_processor;
mod server;

use actix_web::web::get;
use blast::blast;
use config::get_config;
use context::Context;
use model::Model;
use server::main as server_main;

#[tokio::main]
async fn main() {
    let config = get_config();
    let model = Model::new(&config.sqlite_file, &config.feed_url)
        .await
        .expect("Failed to create model");
    let context = Context {
        config: config.clone(),
        model,
    };
    println!("Blasting emails");
    blast(context).await;
}
