
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
mod utils;

use blast::blast;
use config::get_config;
use context::Context;
use model::Model;
use server::main as server_main;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_config();
    let model = Model::new(&config.sqlite_file, &config.feed_url)
        .await
        .expect("Failed to create model");
    let context = Context {
        config: config.clone(),
        model,
    };

    match config.action.as_str() {
        "serve" => {
            println!("Starting server...");
            let _server = server_main(context).await;
        }
        "blast" => {
            println!("Blasting emails");
            let _blast = blast(context).await;
        }
        _ => unreachable!(), // assuming command-line argument parsing ensures this
    }

    Ok(())
}
