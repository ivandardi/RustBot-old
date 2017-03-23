use serenity::client::Context;
use serenity::model::{Member, GuildId, Mentionable};
use serenity::utils::MessageBuilder;

use std::thread;
use std::time::Duration;

pub fn handle(context: Context, guild_id: GuildId, mut member: Member) {
    if guild_id != 273534239310479360 {
        return;
    }

    let welcome_message = MessageBuilder::new()
        .push("Welcome to the ")
        .push_bold("Rust Programming Language ")
        .push(&format!("server, {}!\n", &member.mention()))
        .push("Please take a minute to read the ")
        .push("<#273534239310479360>.\n")
        .push("You'll get permission to speak shortly.")
        .build();

    if let Ok(message) = context.say(&welcome_message) {
        thread::sleep(Duration::from_secs(60));
        let _ = message.delete();
        let _ = member.add_role(273539307254448128);
    }
}
