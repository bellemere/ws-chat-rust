use actix::prelude::*;
use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod actors;

use actors::chat_server::ChatServer;
use actors::chat_session::chat;

async fn check() -> impl Responder {
    "hello"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = ChatServer::default().start();

    HttpServer::new(move || {
        App::new()
            .data(addr.clone())
            .route("/check", web::get().to(check))
            .service(web::resource("/").route(web::get().to(|| {
                HttpResponse::Found()
                    .header("LOCATION", "/static/websocket.html")
                    .finish()
            })))
            .service(web::resource("/ws").to(chat))
            .service(fs::Files::new("/static", "static/"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
