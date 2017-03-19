use serenity::client::Context;
use serenity::model::*;

// TODO: Properly implement the `uptime` command
pub fn exec_cmd(context: &mut Context, message: &Message, args: Vec<String>) -> Result<(), String> {
    let _ = message.reply("1");

    Ok(())
}
