use serenity::client::Context;
use serenity::utils::MessageBuilder;
use serenity::model::Message;
use std::time::{SystemTime, UNIX_EPOCH};

// Required to save the initial startup time of the bot
// as we don't control the arguments to the exec_cmd function
pub static mut START_TIME: SystemTime = UNIX_EPOCH;

command!(uptime(_context, message,_args) {

    // Again, actually safe. The value of START_TIME is not modified
    if let Ok(elapsed) = unsafe { START_TIME.elapsed() } {

        let delta = elapsed.as_secs();
        let (hours, remainder) = (delta / 3600, delta % 3600);
        let (minutes, seconds) = (remainder / 60, remainder % 60);
        let (days, hours) = (hours / 24, hours % 24);

        let uptime_message = MessageBuilder::new()
            .push("Uptime: **")
            .push(days.to_string().as_str())
            .push("d ")
            .push(hours.to_string().as_str())
            .push("h ")
            .push(minutes.to_string().as_str())
            .push("m ")
            .push(seconds.to_string().as_str())
            .push("s")
            .push("**")
            .build();

        let _ = message.channel_id.send_message(|m| m.content(uptime_message.as_str()));
    } else {
        // TODO: log errors
    }

});
