use colored::*;
use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;
use actix::prelude::*;


pub enum TCPMessage {
    Ping,
    PrivMsg(&str),
}

/// Define message
#[rtype(result = "Result<bool, std::io::Error>")]
#[derive(Debug, Clone, Message)]
pub struct ReceivedMessage {
    pub channel: String,
    pub username: String,
    pub user_id: String,
    pub message: String,
}
pub struct TwitchChatConnection {
    pub stream: Option<TcpStream>,
}


impl TwitchChatConnection {
    pub fn new() -> Self {
        TwitchChatConnection{ stream: None }
    }

    pub fn connect_and_authenticate(
        &mut self,
        auth_token: &str,
        chennel_name: &str,
        nickname: &str
    ) -> Result<(), Box<dyn Error>> {
        println!("{}", "Connectin to Twitch".bright_purple());
        let formatted_token = format!("oauth:{}", auth_token);
        let mut stream = TcpStream::connect("irc.chat.twitch.tv:6667")?;
        stream.write_all(
            "CAP REQ :twitch.tv/tags twitch.tv/commands twitch.tv/membership\r\n".as_bytes(),
        )?;
        stream.write_all(format!("PASS {}\r\n", formatted_token).as_bytes())?;
        stream.write_all(format!("NICK {}\r\n", nickname).as_bytes())?;
        stream.write_all(format!("JOIN #{}\r\n", channel_name).as_bytes())?;
    
        self.stream = Some(stream);
        Ok(())
    
    }

    pub fn listen_and_handle_messages(&mut self, channel: String) {
        let stream = match self.stream.as_mut() {
            Ok(stream) => stream,
            Err(_) => {
                println!("{}", "Error Getting Stream".red())
            }
        };

        let mut buffer = Vec::new();

        loop {
            buffer.clear();
            let mut chunk = [0; 512];

            match stream.read(&mut chunk) {
                Ok(bytes_read) => {
                    
                    if bytes_read == 0 {
                        break
                    }

                    buffer.extend_from_slice(&chunk[..bytes_read]);

                    let message_string = String::from_utf8_lossy(&buffer).to_string();

                    println!("{}", "Received message from Twitch:".cyan()); // !REMOVE
                    println!("{}", message_string.yellow()); // !REMOVE

                    let messages = message_string.split("\r\n").collect::<Vec<_>>();

                    let mut message_type: TCPMessage;
                    
                    for message in messages {
                        if message.starts_with("PING") {
                            message_type = TCPMessage::Ping;
                            continue
                        }

                        if message.contains("PRIVMSG") {
                            message_type = TCPMessage::PrivMsg(message);
                            continue
                        }
                    }

                    match message_type {
                        TCPMessage::PrivMsg(message) => {
                            println!("{}", "PrivMsg Recieved!".light_green());
                            if let Some((username, user_id, msg)) = Self::parse_chat_message(message) {
                                let recieved_message = ReceivedMessage {
                                    channel: channle.clone(),
                                    username,
                                    user_id,
                                    message: msg.to_string()
                                };

                                println!("{}", "Recieved Message from Twitch".purple();)
                            }
                        },
                        TCPMessage::Ping => {
                            let _ = Self::send_pong(stream, &message[5..1]);
                            continue
                        }
                    }

                }
                Err(err) => {
                    eprintln!("Error Reading from stream {}", err);
                    break
                }
            }
        }
    }
    fn send_pong(stream: &mut TcpStream, msg: &str) -> Result<(), Box<dyn Error>> {
        stream.write_all(format!("PONG {}\r\n", msg).as_bytes())?;
        Ok(())
    }


    pub fn parse_chat_message(message: &str) -> Option<(String, String, &str)> {
        let parts: Vec<&str> = message.split(" ").collect();

        if let Some(msg_part) = message.split(":").nth(2) {
            let metadata = parts.get(0).unwrap_or(&"");
            let username = metadata
                .split(";")
                .find(|&s| s.starts_with("display-name="))
                .and_then(|s| s.split('=').nth(1))
                .unwrap_or("");

            let user_id = metadata
                .split(";")
                .find(|&s| s.starts_with("user-id="))
                .and_then(|s| s.split('=').nth(1))
                .unwrap_or("");

            Some((username.to_string(), user_id.to_string(), msg_part))
        } else {
            None
        }
    }
}






