use serenity::client::Context;
use serenity::model::*;
use serenity::utils::MessageBuilder;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Required to save the initial startup time of the bot
// as we don't control the arguments to the exec_cmd function
pub static mut START_TIME: SystemTime = UNIX_EPOCH;

pub fn exec_cmd(_context: &mut Context,
                message: &Message,
                _args: Vec<String>)
                -> Result<(), String> {
    let mut uptime_message = MessageBuilder::new().push("Uptime: **");
    let mut elapsed_time = Ok(Duration::new(0, 0));

    // Again, actually safe. The value of START_TIME is not modified
    unsafe {
        elapsed_time = START_TIME.elapsed();
    }

    match elapsed_time {
        Ok(elapsed) => {
            let seconds = elapsed.as_secs();
            let (mins, hours, days) = (seconds / 60 % 60, seconds / 3_600, seconds / 86_400);

            if days != 0 {
                uptime_message = uptime_message.push(days.to_string().as_str()).push("d ");
            }

            uptime_message = uptime_message.push(hours.to_string().as_str()).push("h ");
            uptime_message = uptime_message.push(mins.to_string().as_str()).push("m ");
            uptime_message = uptime_message.push((seconds % 60).to_string().as_str()).push("s");
            uptime_message = uptime_message.push("**");

            let _ = message.channel_id.send_message(|m| m.content(uptime_message.build().as_str()));
        }
        Err(e) => {
            // TODO: log errors
        }
    }

    Ok(())
}
