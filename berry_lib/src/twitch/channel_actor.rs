use super::twitch_chat::{ReceivedMessage, TwitchChatConnection};
use actix::prelude::*;
use actix::dev::MessageResponse;
use colored::*;
use std::io::{BufReader, Error, ErrorKind};


pub struct ChannelActor {
    channel: String,
    nickname: String,
    auth_token: String,
    chat_connection: TwitchChatConnection
}


impl ChannelActor {
    pub fn new(channel: String, nickname: String, auth_token: String) -> Self {
        ChannelActor {
            channel,
            nickname,
            auth_token,
            chat_connection: TwitchChatConnection::new(),
        }
    }
}

impl<A, M> MessageResponse<A, M> for ReceivedMessage <bool, Error>
where
    A: Actor,
    M: Message<Result = Result<bool, Error>>,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}



impl Actor for ChannelActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("{} {}", "Started Channel Actor for ".cyan(), &self.channel);

        match self.chat_connection.connect_and_authenticate(&self.auth_token, &self.channel_name, &self.nickname) {
            Ok(_) => {
                let stream = match self.chat_connection.stream.as_mut() {
                    Some(stream) => stream,

                    None => {
                        println!("{}", "Error Getting Stream".red());
                        return;
                    }
                };

                let reader = BufReader::new(stream);

                ctx.add_stream(reader);
            }
            Err(e) => {
                println!("Error Connecting to Twitch: {}", e);
                ctx.stop();
            }
        }
    }
}


impl Handler<ReceivedMessage> for ChannelActor {
    type Result = Result<bool, Error>;

    fn handle(&mut self, msg: ReceivedMessage, ctx: &mut Self::Context) {
        println!("{}: {}", msg.username, msg.message);
        Ok(true)
    }
}