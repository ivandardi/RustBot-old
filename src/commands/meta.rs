use serenity::utils::{ MessageBuilder, Colour };
use serenity::model::{ User, UserId };

use time::{ Tm, TmFmt, at_utc };
use typemap_kv::UptimerKey;
use uptimer::Uptimer;

command!(uptime(context, _msg, _args) {
    let data = context.data.lock().unwrap();
    let uptimer = data.get::<UptimerKey>().expect("Failed to get Uptimer");

    if let Some(channel) = context.channel_id {
        channel.send_message(|cm| cm.embed(|ce|
            ce.title("Uptime")
              .description(&uptimer.uptime_string())
        ));
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