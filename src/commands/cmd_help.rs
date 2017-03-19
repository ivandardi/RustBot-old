use serenity::client::Context;
use serenity::ext::framework::*;
use serenity::ext::framework::CommandOrAlias::{Command, Alias};
use serenity::model::*;
use serenity::utils::MessageBuilder;

use std::collections::HashMap;
use std::sync::Arc;

pub fn exec_cmd(context: &mut Context,
                message: &Message,
                cmds: HashMap<String, Arc<CommandGroup>>,
                args: Vec<String>)
                -> Result<(), String> {

    // We're a default help call
    if args.len() == 0usize {

        // Begin building the message to reply with
        let mut default_help_string =
            MessageBuilder::new().push("```\nHello! I am the community RustBot that \
                                        assists in managing the Rust server!\n\nMeta:\n");

        // Iterate through the CommandGroup to grab all of the added commands,
        // then append them to the help string
        for (_name, cmdgrp) in &cmds {
            for (name, command) in &cmdgrp.clone().commands {
                match *command {
                    Command(ref x) => {
                        match x.desc {
                            Some(ref desc_str) => {
                                default_help_string =
                                    default_help_string.push("\t").push(name.as_str()).push("\t\t");
                                default_help_string = default_help_string.push(desc_str.as_str());
                            }
                            None => {}
                        }
                    }
                    Alias(ref _alias) => {}
                }
            }
        }

        // Finish up the help string
        default_help_string =
            default_help_string.push("\n\nType ?help command for more info on a command.\n")
                    .push("You can also type ?help category for more info on a category.```");

        if let Err(why) = message.author.direct_message(default_help_string.build().as_str()) {
            // TODO: add proper logging
        }

    }

    // TODO: Add support for arguments

    Ok(())
}
