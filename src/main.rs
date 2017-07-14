#![recursion_limit = "1024"]

#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serenity;
extern crate toml;
extern crate time;
extern crate typemap;
extern crate num_integer;

mod error;
mod config;
mod logging;
mod commands;
mod uptimer;

use error::Result;
use config::Config;
use logging::Logger;
use uptimer::{UptimerKey, Uptimer};

use serenity::prelude::*;
use serenity::ext::framework::help_commands;

struct Handler;

impl EventHandler for Handler {}

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

    let config = Config::from_file("config.toml")?;

    let mut client = Client::new(&config.token, Handler);

    {
        let mut data = client.data.lock();
        data.insert::<UptimerKey>(Uptimer::new());
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    client.with_framework(|f| f
        .configure(|c| c
            .prefix("??")
            .on_mention(true))
        .before(|_ctx, msg, command_name| {
            info!("{} in {}: {}", msg.author.name, msg.channel_id.name().expect("Channel Name"), command_name);
            true
        })
        .group("Help", |g| g
            .command("help", |c| c
                .exec_help(help_commands::with_embeds)
                .desc("Displays a list of the bots command with embeds."))
            .command("helpp", |c| c
                .exec_help(help_commands::plain)
                .desc("Displays a list of the bots command in plain text.")))
        .group("Meta", |g| g
            .command("ping", |c| c
                .exec(commands::ping)
                .desc("Checks for connection speed."))
            .command("uptime", |c| c
                .exec(commands::uptime)
                .desc("Tells you how long the bot has been up for."))
            .command("info", |c| c
                .exec(commands::info)
                .desc("Displays member info")))
    );

    client.start()?;

    Ok(())
}
