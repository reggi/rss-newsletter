use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::Arc;
use crate::context::Context;

#[derive(Deserialize)]
pub struct SubscribeRequest {
    email: String,
}

#[derive(Deserialize)]
pub struct UnsubscribeRequest {
    email: String,
}

#[derive(Deserialize)]
pub struct UnsubscribeQuery {
    email: Option<String>,
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

pub async fn subscribe(
    context: web::Data<Arc<Context>>,
    form: web::Form<SubscribeRequest>
) -> impl Responder {
    let res = context.model.add_subscriber(&form.email).await;
    match res {
        Ok(_) => HttpResponse::Ok().body("Subscribed successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to subscribe: {}", e))
    }
}

pub async fn unsubscribe(
    context: web::Data<Arc<Context>>,
    form: web::Form<UnsubscribeRequest>
) -> impl Responder {
    let res = context.model.unsubscribe(&form.email).await;
    match res {
        Ok(_) => HttpResponse::Ok().body("Unsubscribe was successful"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to unsubscribe: {}", e))
    }
}

pub async fn subscribe_form() -> impl Responder {
    let html = r#"
        <html>
            <body>
                <form action="/api/subscribe" method="post">
                    <input type="text" name="email" />
                    <button type="submit">Subscribe</button>
                </form>
            </body>
        </html>
    "#;

    HttpResponse::Ok().content_type("text/html").body(html)
}

pub async fn unsubscribe_form(query: web::Query<UnsubscribeQuery>) -> impl Responder {
    let email = query.email.as_ref().map_or("", String::as_str);
    let html = format!(
        r#"
        <html>
            <body>
                <form action="/api/unsubscribe" method="post">
                    <input type="text" name="email" value="{}" />
                    <button type="submit">Unsubscribe</button>
                </form>
            </body>
        </html>
    "#,
        email // This inserts the email into the form value if present
    );

    HttpResponse::Ok().content_type("text/html").body(html)
}