use crate::actors::chat_server::Connect;
use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::time::Duration;

use crate::actors::chat_server::{ChatServer, ServerMessage};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

#[derive(Debug)]
pub struct ChatSession {
    id: usize,
    addr: Addr<ChatServer>,
}

impl ChatSession {
    fn heart_beat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |_, ctx| {
            ctx.ping(b"");
        });
    }
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("----------started");
        self.heart_beat(ctx);

        let addr = ctx.address();
        self.addr
            .send(Connect { addr })
            .into_actor(self)
            .then(|res, act, ctx| {
                println!("{:?}", res);
                match res {
                    Ok(id) => act.id = id,
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }
}

fn message_consumer(
    text: String,
    act: &mut ChatSession,
    _: &mut ws::WebsocketContext<ChatSession>,
) {
    println!("----------->:{:?}", text);
    act.addr.do_send(ServerMessage {
        id: act.id,
        msg: text,
    })
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

#[derive(Message)]
#[rtype(result = "()")]
pub struct SessionMessage {
    pub msg: String,
}

impl Handler<SessionMessage> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: SessionMessage, ctx: &mut Self::Context) -> Self::Result {
        println!("~~~~~~~~~~~~~~~~~~~~~{:?}", msg.msg);
        ctx.text(msg.msg);
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
