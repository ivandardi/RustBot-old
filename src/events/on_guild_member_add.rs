use serenity::client::Context;
use serenity::model::*;

use std::thread;
use std::time::Duration;

pub fn handle(context: Context, guild_id: GuildId, mut member: Member) {
    if guild_id != 273534239310479360 {
        return;
    }
    let msg = context.say(&format!("Welcome to the Rust server {}! \
                                   Please take a minute to read the #rules.",
                                   member.mention()));
    thread::sleep(Duration::from_secs(60));
    let _ = member.add_role(273539307254448128);
    if let Ok(msg) = msg {
        let _ = msg.delete();
    }
}
