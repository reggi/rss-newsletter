// #![allow(warnings)]

use rss_newsletter::config::get_config;
use rss_newsletter::context::Context;
use rss_newsletter::model::Model;
use rss_newsletter::server::main as server_main;

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
    server_main(context).await
}
