use std::time::Duration;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use tera::{Context, Tera};

#[get("/")]
async fn hello() -> HttpResponse {
    let mut context = Context::new();
    context.insert("context", "Rust Index");
    let body = Tera::one_off(include_str!("templates/index.html"), &context, false)
        .expect("Failed to render template");
    HttpResponse::Ok().body(body)
}

#[actix_web::main()]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| App::new().service(hello))
        .bind_openssl("127.0.0.1:8080", builder)?
        .keep_alive(Duration::from_secs(60))
        .run()
        .await
}
