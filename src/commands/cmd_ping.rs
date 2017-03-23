use serenity::client::Context;
use serenity::model::Message;

command!(ping(context,_message,_args) {
    if let Some(channel) = context.channel_id {
    let _ = channel.send_message( | m | m.content("Pong!"));
    }
});
