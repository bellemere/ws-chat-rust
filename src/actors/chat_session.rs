use actix::{Actor, Addr, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::actors::chat_server::ChatServer;

pub struct ChatSession {
    id: usize,
    addr: Addr<ChatServer>,
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;
}

fn message_consumer(
    text: String,
    cs: &mut ChatSession,
    ctx: &mut ws::WebsocketContext<ChatSession>,
) {
    println!("----------->:{:?}", text);
    ctx.text(text);
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => message_consumer(text, self, ctx),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub async fn chat(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(
        ChatSession {
            id: 0,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    );
    println!("{:?}", resp);
    resp
}
