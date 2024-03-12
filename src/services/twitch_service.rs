use berry_lib::twitch::twitch_chat::{TwitchBot, TwitchChatConnection};
use berry_lib::twitch::bot_storage;
use berry_lib::twitch::message_processor;
use tokio::sync::broadcast::Receiver;
use berry_lib::twitch::twitch_chat::ReceivedMessage;

pub async fn start_bot(channel: String, nickname: String, auth_token: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let connection = TwitchChatConnection::connect_and_authenticate(&auth_token, &channel, &nickname).await?;
    let receiver = connection.sender.subscribe();
    let mut bot = TwitchBot {
        channel: channel.clone(),
        nickname,
        auth_token,
        chat_connection: connection,
    };
    bot_storage::add_bot(channel.clone(), bot.clone());
    tokio::spawn(async move {
        bot.chat_connection.listen_and_handle_messages(channel.to_string()).await;
    });
    handle_messages(receiver).await;
    Ok(())
}

async fn handle_messages(mut receiver: Receiver<ReceivedMessage>) {
    while let Ok(message) = receiver.recv().await {
        message_processor::process_message(message).await;
    }
}