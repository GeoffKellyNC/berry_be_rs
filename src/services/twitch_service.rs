use berry_lib::twitch::twitch_chat::TwitchBot;
use berry_lib::twitch::bot_storage;

pub fn start_bot(bot: TwitchBot) {
    let TwitchBot {
        channel,
        nickname,
        auth_token,
        mut chat_connection,
    } = bot;

    match chat_connection.connect_and_authenticate(&auth_token, &channel, &nickname) {
        Ok(()) => {
            bot_storage::add_bot(channel.clone(), bot);
            chat_connection.listen_and_handle_messages(channel.to_string());
        }
        Err(_) => {
            println!("Error Starting Bot");
        }
    }
}