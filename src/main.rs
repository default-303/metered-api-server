mod database;
mod mpsc_bridge;
mod services;

use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use tokio::sync;

use std::io;

use services::{get_data, hola, register_client};

#[actix_web::main]
async fn main() -> io::Result<()> {
    let (sender, receiver) = sync::mpsc::channel(256);
    tokio::spawn(async move { mpsc_bridge::bridge(receiver).await });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(sender.clone()))
            .service(hola)
            .service(register_client)
            .service(get_data)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
