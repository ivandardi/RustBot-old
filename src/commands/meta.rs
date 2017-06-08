use serenity::utils::Colour;
use typemap_kv::UptimerKey;

command!(uptime(ctx, msg) {
    let uptime_string = {
        let data = ctx.data.lock().expect("Failed to accuire lock");
        let uptimer = data.get::<UptimerKey>().expect("Failed to get Uptimer");
        uptimer.uptime_string()
    };
    let result = msg.channel_id.send_message(|cm| 
        cm.embed(|ce|
            ce.title("Uptime")
              .description(&uptime_string)
              .color(Colour::blue())
        )
    );
    if result.is_err() {
        return Err("Failed send message".to_owned());
    }
});

command!(ping(_ctx, msg) {
    let result = msg.channel_id.send_message(|m| m.content("Pong!"));
    if result.is_err() {
        return Err("Failed to send message".to_owned());
    }
});

command!(memberinfo(_ctx, msg) {
    if let Some(guild) = msg.guild() {
        let guild = guild.read().expect("Failed to accuire read lock");
        if let Some(member) = guild.members.get(&msg.author.id) {
            let mut roles = String::new();
            let mut iter = member.roles.iter().peekable();
            while let Some(role_id) = iter.next() {
                if let Some(role) = role_id.find() {
                    roles.push_str(&role.name);
                } else {
                    // NOTE: I'm unsure if it's an error when role_id doesn't have corresponding 
                    // role. For now I consider this error. 
                    return Err("Failed to get Role for RoleId".to_owned());
                }

                if iter.peek().is_some() {
                    roles.push_str(", ");
                }
            }
            let joined_at = {
                if let Some(joined_at) = member.joined_at.as_ref() {
                    joined_at.naive_utc().format("%c")
                } else {
                    return Err("Failed to get Member's joined at".to_owned());
                }
            }; 
            let avatar_url = msg.author.face();
            let id = msg.author.id.0.to_string();
            let nick = member.nick.as_ref().unwrap_or_else(|| &msg.author.name);
            let dtag = msg.author.tag();
            let created_at = msg.author.created_at().format("%c").to_string();
            let footer_text = format!("Member since {}", joined_at);
            let result = msg.channel_id.send_message(|cm| cm.embed(|ce| 
                ce.author(|cea| cea.name(&dtag).icon_url(&avatar_url))
                .field(|cef| cef.inline(true).name("Id").value(&id))
                .field(|cef| cef.inline(true).name("Current name").value(nick))
                .field(|cef| cef.inline(true).name("Created at").value(&created_at))
                .field(|cef| cef.inline(true).name("Roles").value(&roles))
                .footer(|cef| cef.text(&footer_text))
                .image(&avatar_url)
                .color(Colour::blue())
            ));
            if result.is_err() {
                return Err("Failed to send message".to_owned())
            }
        }
    }
});