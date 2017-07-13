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
mod typemap_kv;

use error::Result;
use config::Config;
use logging::Logger;
use uptimer::Uptimer;
use typemap_kv::*;

use serenity::client::Client;
use serenity::ext::framework::help_commands;

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

    let mut client = Client::new(&config.token);

    {
        let mut data = client.data.lock().unwrap();
        data.insert::<UptimerKey>(Uptimer::new());
    }

    client.with_framework(|f| f
        .configure(|c| c
            .prefix("??"))
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
                .desc("Tells you how long the bot has been up for.")))
            .command("info", |c| c
                .exec(commands::info)
                .desc("Displays member info"))
            .command("permissions", |c| c
                .exec(commands::permissions)
                .desc("Displays member permissions"))
    );

    client.start()?;

    Ok(())
}
