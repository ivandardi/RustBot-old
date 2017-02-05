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
mod events;
mod commands;

use error::Result;
use config::Config;
use logging::Logger;

use serenity::client::Client;

fn main() {
    std::process::exit(match actual_main() {
        Ok(_) => 0,
        Err(err) => {
            error!("Error in main: {}", err);
            1
        }
    });
}

fn actual_main() -> Result<()> {
    // todo: add commands
    Logger::init()?;
    let config = Config::from_file("config.json")?;
    let mut client = Client::login_bot(&config.token);
    client.on_guild_member_add(events::on_member_join);
    client.start()?;
    Ok(())
}
