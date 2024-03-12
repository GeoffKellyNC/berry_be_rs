use berry_lib::twitch::twitch_chat::TwitchBot;
use berry_lib::twitch::message_processor::MessageProcessor;
use berry_lib::twitch::bot_storage;
use std::sync::Arc;

pub fn start_bot(bot: &mut TwitchBot, message_processor: MessageProcessor) {
    let TwitchBot {
        channel,
        nickname,
        auth_token,
        chat_connection,
    } = bot;

    match chat_connection.connect_and_authenticate(&auth_token, &channel, &nickname) {
        Ok(()) => {
            let bot_arc = Arc::new(bot.clone());
            bot_storage::add_bot(channel.clone(), bot_arc.clone());
            chat_connection.listen_and_handle_messages(channel.to_string(), message_processor);
        }
        Err(_) => {
            println!("Error Starting Bot");
        }
    }
}
 
