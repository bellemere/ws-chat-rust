//use actix::{Actor, Addr, Context, Handler, Message};
use actix::prelude::*;
use std::collections::HashMap;

use crate::actors::chat_session::ChatSession;

pub struct ChatServer {
    sessions: HashMap<String, Addr<ChatSession>>,
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
}

#[derive(Message)]
#[rtype(result = "usize")]
pub struct Connect;

impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        1
    }
}
