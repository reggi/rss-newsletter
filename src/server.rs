use crate::routes::{index, subscribe, subscribe_form, unsubscribe, unsubscribe_form};
use crate::types::ServerContext as Context;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;

pub async fn main(context: Context) -> std::io::Result<()> {
    let address = format!("127.0.0.1:{}", context.config.port);
    let shared_context = Arc::new(context.clone());

    // Set up the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_context.clone()))
            .route("/", web::get().to(index)) // Define routes and handlers
            .route("/subscribe", web::get().to(subscribe_form))
            .route("/unsubscribe", web::get().to(unsubscribe_form))
            .route("/api/subscribe", web::post().to(subscribe))
            .route("/api/unsubscribe", web::post().to(unsubscribe))
    })
    .bind(&address)? // Bind the server to the specified address and port
    .run() // Run the server
    .await
}
