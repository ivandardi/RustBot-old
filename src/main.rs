#![recursion_limit = "1024"]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
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
use serenity::ext::framework::help_commands;

use std::time::SystemTime;
use commands::START_TIME;

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

    // Required to edit static variables,
    // no actual unsafety as we only ever read the value
    unsafe {
        START_TIME = SystemTime::now();
    }

    let config = Config::from_file("config.json")?;
    let mut client = Client::login_bot(&config.token);

    client.on_guild_member_add(events::on_member_join);

    client.with_framework(|f| f
        .configure(|c| c
            .prefix("??"))
        .group("Help", |g| g
            .command("help", |c| c
                .exec_help(help_commands::with_embeds))
            .command("helpp", |c| c
                .exec_help(help_commands::plain)))
        .group("Meta", |g| g
            .command("ping", |c| c
                .exec(commands::ping)
                .desc("Checks for connection speed."))
            .command("uptime", |c| c
                .exec(commands::uptime)
                .desc("Tells you how long the bot has been up for.")))
    );

    client.start()?;

    Ok(())
}
