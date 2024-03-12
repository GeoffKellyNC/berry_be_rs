use colored::*;
use std::error::Error;
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast::{self, Sender};

pub enum TCPMessage {
    Ping(String),
    PrivMsg(String),
}

/// Define message
#[derive(Debug, Clone)]
pub struct ReceivedMessage {
    pub channel: String,
    pub username: String,
    pub user_id: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct TwitchChatConnection {
    pub stream: Arc<Mutex<TcpStream>>,
    pub sender: Sender<ReceivedMessage>,
}

impl TwitchChatConnection {
    pub fn new(stream: TcpStream) -> Self {
        let (sender, _) = broadcast::channel(100);
        TwitchChatConnection {
            stream: Arc::new(Mutex::new(stream)),
            sender,
        }
    }

    pub async fn connect_and_authenticate(
        auth_token: &str,
        channel_name: &str,
        nickname: &str,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        println!("{}", "Connecting to Twitch".bright_purple());
        let formatted_token = format!("oauth:{}", auth_token);
        let stream = TcpStream::connect("irc.chat.twitch.tv:6667")?;
        let mut connection = Self::new(stream);
        connection.write_all(
            "CAP REQ :twitch.tv/tags twitch.tv/commands twitch.tv/membership\r\n".as_bytes(),
        ).await?;
        connection.write_all(format!("PASS {}\r\n", formatted_token).as_bytes()).await?;
        connection.write_all(format!("NICK {}\r\n", nickname).as_bytes()).await?;
        connection.write_all(format!("JOIN #{}\r\n", channel_name).as_bytes()).await?;
        println!("{}", "Authenticated with Twitch".bright_purple());
        println!("Connection: {:?}", connection); // !REMOVE
        Ok(connection)
    }

    async fn write_all(&mut self, data: &[u8]) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut stream = self.stream.lock().unwrap();
        stream.write_all(data)?;
        Ok(())
    }

    pub async fn listen_and_handle_messages(&mut self, channel: String) {
        println!("{} {}", "Listening for Messages in: ".bright_purple(), &channel);
        let mut buffer = Vec::new();
        loop {
            buffer.clear();
            let mut chunk = [0; 512];
            {
                let mut stream = self.stream.lock().unwrap();
                let mut stream_reader = BufReader::new(&mut *stream);
                match stream_reader.read(&mut chunk) {
                    Ok(bytes_read) => {
                        if bytes_read == 0 {
                            break;
                        }
                        buffer.extend_from_slice(&chunk[..bytes_read]);
                        let message_string = String::from_utf8_lossy(&buffer).to_string();
                        println!("{}", "Received message from Twitch:".cyan());
                        println!("{}", message_string.yellow());
                        let messages = message_string.split("\r\n").collect::<Vec<_>>();
                        for message in messages {
                            let message_type = match message {
                                message if message.starts_with("PING") => TCPMessage::Ping(message.to_string()),
                                message if message.contains("PRIVMSG") => TCPMessage::PrivMsg(message.to_string()),
                                _ => continue,
                            };
                            match message_type {
                                TCPMessage::Ping(message) => {
                                    let pong_message = format!("PONG {}\r\n", &message[5..]);
                                    if let Err(e) = stream.write_all(pong_message.as_bytes()) {
                                        eprintln!("Error sending PONG: {}", e);
                                    }
                                }
                                TCPMessage::PrivMsg(message) => {
                                    println!("{}", "PrivMsg Received!".green());
                                    if let Some((username, user_id, msg)) = Self::parse_chat_message(&message) {
                                        let received_message = ReceivedMessage {
                                            channel: channel.clone(),
                                            username,
                                            user_id,
                                            message: msg.to_string(),
                                        };
                                        println!("{} {:?}", "Received Message from Twitch".purple(), received_message);
                                        if let Err(e) = self.sender.send(received_message) {
                                            eprintln!("Failed to send message to queue: {}", e);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Error reading from stream: {}", err);
                        break;
                    }
                }
            }
        }
    }

    async fn write_pong(&mut self, pong_message: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.write_all(pong_message.as_bytes()).await
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

    pub async fn send_chat_message(&mut self, channel: &str, message: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        let msg = format!("PRIVMSG #{} :{}\r\n", channel, message);
        self.write_all(msg.as_bytes()).await?;
        println!("{}", "Message sent to chat".green());
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct TwitchBot {
    pub channel: String,
    pub nickname: String,
    pub auth_token: String,
    pub chat_connection: TwitchChatConnection,
}