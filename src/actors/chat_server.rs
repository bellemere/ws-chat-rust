use actix::prelude::*;
use rand::{self, Rng};
use std::collections::HashMap;

use crate::actors::chat_session::{ChatSession, SessionMessage};

#[derive(Debug)]
pub struct ChatServer {
    sessions: HashMap<usize, Addr<ChatSession>>,
}

impl ChatServer {
    fn send_message(&self, msg: SessionMessage, skip_id: usize) {
        for (id, addr) in &self.sessions {
            if *id != skip_id {
                addr.do_send(SessionMessage {
                    msg: msg.msg.to_owned(),
                });
            }
        }
    }
}

impl Default for ChatServer {
    fn default() -> ChatServer {
        ChatServer {
            sessions: HashMap::new(),
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("ChatServer is started");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("ChatServer is stopped");
    }
}

#[derive(Message)]
#[rtype(result = "usize")]
pub struct Connect {
    pub addr: Addr<ChatSession>,
}

impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let rand = rand::thread_rng().gen::<usize>();
        println!("connect message received: {}", rand);
        self.sessions.insert(rand, msg.addr);
        self.send_message(
            SessionMessage {
                msg: "Someone joined".to_string(),
            },
            rand,
        );
        rand
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ServerMessage {
    pub id: usize,
    pub msg: String,
}

impl Handler<ServerMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage, _: &mut Context<Self>) -> Self::Result {
        self.send_message(
            SessionMessage {
                msg: msg.msg.to_string(),
            },
            0,
        );
    }
}
