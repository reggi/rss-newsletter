use rss_newsletter::config::get_config;

#[tokio::main]
async fn main() {
    let config = get_config();
    println!("Configuration parsed: {:?}", config);
}