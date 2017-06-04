use serenity::utils::{ MessageBuilder, Colour };
use serenity::model::{ User, UserId };

use std::time::{SystemTime, UNIX_EPOCH};
use time::{ Tm, TmFmt, at_utc };

// Required to save the initial startup time of the bot
// as we don't control the arguments to the exec_cmd function
pub static mut START_TIME: SystemTime = UNIX_EPOCH;

command!(uptime(_context, message,_args) {

    // Again, actually safe. The value of START_TIME is not modified
    if let Ok(elapsed) = unsafe { START_TIME.elapsed() } {

        let delta = elapsed.as_secs();
        let (hours, remainder) = (delta / 3600, delta % 3600);
        let (minutes, seconds) = (remainder / 60, remainder % 60);
        let (days, hours) = (hours / 24, hours % 24);

        let uptime_message = MessageBuilder::new()
            .push("Uptime: **")
            .push(days.to_string().as_str())
            .push("d ")
            .push(hours.to_string().as_str())
            .push("h ")
            .push(minutes.to_string().as_str())
            .push("m ")
            .push(seconds.to_string().as_str())
            .push("s")
            .push("**")
            .build();

        let _ = message.channel_id.send_message(|m| m.content(uptime_message.as_str()));
    } else {
        // TODO: log errors
    }

});


command!(ping(context,_message,_args) {
    if let Some(channel) = context.channel_id {
    let _ = channel.send_message( | m | m.content("Pong!"));
    }
});

command!(memberinfo(_context, msg, _args) {
    let avatar_url = msg.author.avatar_url().or_else(||
        msg.author.static_avatar_url()
    ).unwrap_or_else(|| 
        msg.author.default_avatar_url()
    );
    let id = msg.author.id.0;
    let ref name = msg.author.name;
    let dtag = msg.author.distinct();
    let created = at_utc(msg.author.created_at());
    let created = created.asctime();
    let member_info = msg.guild().and_then(|guild| {
        let guild = guild.read().expect("Failed to accure read lock");
        guild.members.get(&msg.author.id).and_then(|member| {
            let mut roles = String::new();
            let mut iter = member.roles.iter().peekable();
            while let Some(role_id) = iter.next() {
                if let Some(role) = role_id.find() {
                    roles.push_str(&role.name);
                } else {
                    return None;
                }

                if iter.peek().is_some() {
                    roles.push_str(", ");
                }
            }
            Some((roles, member.joined_at.clone()))
        })
    });

    if let Some((roles, member_since)) = member_info {
        msg.channel_id.send_message(|cm| cm.embed(|ce| 
            ce.author(|cea|
                cea.name(&dtag)
                .icon_url(&avatar_url)
            )
            .field(|cef| 
                cef.inline(true)
                .name("Id")
                .value(&format!("{}", id))
            )
            .field(|cef| 
                cef.inline(true)
                .name("Current name")
                .value(name)
            )
            .field(|cef| 
                cef.inline(true)
                .name("Created at")
                .value(&format!("{}", created))
            )
            .field(|cef|
                cef.inline(true)
                .name("Roles")
                .value(&roles)
            )
            .footer(|cef| 
                cef.text(&format!("Member since {}", member_since))
            )
            .image(&avatar_url)
        ));
    } else {
        msg.channel_id.send_message(|cm| cm.embed(|ce|
            ce.title("Error")
              .description(
                  "Something really bad has happened during command execution"
               )
        ));
    }
});