// twitch_api.rs
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub struct TwitchMessage {
    pub sender: String,
    pub text: String,
}

impl Default for TwitchMessage {
    fn default() -> Self {
        TwitchMessage {
            sender: String::new(),
            text: String::new(),
        }
    }
}

#[derive(Debug)]
pub enum TwitchError {
    IOError(std::io::Error),
    ConnectionError,
    MessageParseError,
}

impl From<std::io::Error> for TwitchError {
    fn from(err: std::io::Error) -> Self {
        TwitchError::IOError(err)
    }
}

pub struct TwitchAPI<'a> {
    access_token: &'a str,
    channel: &'a str,
    stream: Option<TcpStream>,
    reader: Option<BufReader<TcpStream>>,
}

impl<'a> TwitchAPI<'a> {
    pub fn new(access_token: &'a str, channel: &'a str) -> Result<Self, TwitchError> {
        Ok(TwitchAPI {
            access_token,
            channel,
            stream: None,
            reader: None,
        })
    }

    pub fn connect(&mut self) -> Result<(), TwitchError> {
        let stream = TcpStream::connect("irc.chat.twitch.tv:6667").map_err(|_| TwitchError::ConnectionError)?;
        stream.set_nonblocking(true)?;

        let mut writer = stream.try_clone()?;
        writer.write_all(format!("PASS oauth:{}\r\n", self.access_token).as_bytes())?;
        writer.write_all(format!("NICK bot_username\r\n").as_bytes())?;
        writer.write_all(format!("JOIN #{}\r\n", self.channel).as_bytes())?;

        self.stream = Some(stream);
        self.reader = Some(BufReader::new(self.stream.as_ref().unwrap().try_clone()?));
        println!("Connected to Twitch: {}", self.channel); // !REMOVE
        Ok(())
    }

    pub fn read_message(&mut self) -> Result<Option<TwitchMessage>, TwitchError> {
        if let Some(reader) = &mut self.reader {
            let mut line = String::new();
            if reader.read_line(&mut line)? > 0 {
                if line.starts_with("PING") {
                    self.send_raw_message("PONG :tmi.twitch.tv\r\n")?;
                } else if line.contains("PRIVMSG") {
                    let parts: Vec<&str> = line.split(' ').collect();
                    if parts.len() >= 4 {
                        let sender = parts[0][1..].split('!').next().unwrap_or("").to_string();
                        let text = parts[3..].join(" ")[1..].trim().to_string();
                        return Ok(Some(TwitchMessage { sender, text }));
                    }
                    return Err(TwitchError::MessageParseError);
                }
            }
        }
        Ok(None)
    }

    pub fn send_message(&mut self, message: &str) -> Result<(), TwitchError> {
        self.send_raw_message(&format!("PRIVMSG #{} :{}\r\n", self.channel, message))
    }

    fn send_raw_message(&mut self, message: &str) -> Result<(), TwitchError> {
        println!("Sending message: {}", message); // !REMOVE
        if let Some(stream) = &mut self.stream {
            stream.write_all(message.as_bytes())?;
        }
        Ok(())
    }

    pub fn disconnect(&mut self) -> Result<(), TwitchError> {
        self.send_raw_message(&format!("PART #{}\r\n", self.channel))?;
        if let Some(stream) = &mut self.stream {
            stream.shutdown(std::net::Shutdown::Both)?;
        }
        Ok(())
    }
}