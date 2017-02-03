#![recursion_limit = "1024"]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate serenity;

extern crate serde_json;
extern crate serde;

mod error;
mod config;
mod logging;

use error::Result;
use config::Config;
use logging::Logger;

use serenity::client::Client;
use serenity::model::*;

fn main() {
    std::process::exit(match actual_main() {
        Ok(_) => 0,
        Err(err) => {
            error!("Error: {}", err);
            1
        }
    });
}

fn actual_main() -> Result<()> {
    // todo: add commands
    Logger::init()?;
    let config = Config::from_file("config.json")?;
    let mut client = Client::login_bot(&config.token);
    client.on_guild_member_add(|context, guild_id, mut member| {
        if guild_id != 273534239310479360 {
            return;
        }
        let msg =
            context.say(&format!("Welcome to the Rust server {}! Please take a minute to read \
                                  the #rules.",
                                 member.mention()));
        std::thread::sleep(std::time::Duration::from_secs(60));
        let _ = member.add_role(273539307254448128);
        if let Ok(msg) = msg {
            let _ = msg.delete();
        }
    });
    client.start()?;
    Ok(())
}
