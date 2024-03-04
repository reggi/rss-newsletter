// #![allow(warnings)]

use rssnewsletter::config::get_config;
use rssnewsletter::context::Context;
use rssnewsletter::model::Model;
use rssnewsletter::server::main as server_main;

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
