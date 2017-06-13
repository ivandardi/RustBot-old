use serenity::utils::Colour;
use serenity::model::permissions::Permissions;
use serenity::model::RoleId;
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

command!(info(_ctx, msg) {
    if let Some(guild) = msg.guild() {
        let guild = guild.read().expect("Failed to accuire read lock");
        if let Some(member) = guild.members.get(&msg.author.id) {
            // NOTE: \u200b is zero-width whitespace.
            let mut roles = "@every\u{200b}one".to_owned(); 
            let mut iter = member.roles.iter();
            while let Some(role_id) = iter.next() {
                if let Some(role) = role_id.find() {
                    roles.push_str(", ");
                    roles.push_str(&role.name);
                } else {
                    // NOTE: I'm unsure if it's an error when role_id doesn't have corresponding 
                    // role. For now I consider this error. 
                    return Err("Failed to get Role for RoleId".to_owned());
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
                .title("Info")
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

command!(permissions(_ctx, msg) {
    if let Some(guild) = msg.guild() {
        let guild = guild.read().expect("Failed to accuire Guild RwLock");
        if let Some(member) = guild.members.get(&msg.author.id) {
            let member_user_id = {
                let user = member.user.read().expect("Failed to accuire User RwLock");
                user.id
            };
            let permissions = if guild.owner_id == member_user_id {
                Permissions::all()
            } else {
                // NOTE: Next line is unobvious at first glance. Actually it gets @everyone
                // role permissions. This role promised to have same id as guild.
                let mut permissions = guild.roles.get(&RoleId(guild.id.0)).unwrap().permissions;
                for role_id in &member.roles {
                    if let Some(role) = role_id.find() {
                        permissions |= role.permissions;
                    } else {
                        // NOTE: I'm unsure if it's an error when role_id doesn't have corresponding 
                        // role. For now I consider this error. 
                        return Err("Failed to get Role for RoleId".to_owned());
                    }
                }
                if permissions.administrator() {
                    permissions = Permissions::all();
                }
                permissions
            };
            
            let result = msg.channel_id.send_message(|cm| cm.embed(|ce|
                ce.title("Permissions")
                  .color(Colour::blue())
                  .description(
                    // TODO: Do something with this ugly format macro invocation.
                    &format!(
                        concat!(
                            "Add reactions: {}\n",
                            "Administrator: {}\n",
                            "Attach files: {}\n",
                            "Ban members: {}\n",
                            "Change nickname: {}\n",
                            "Connect: {}\n",
                            "Create invite: {}\n",
                            "Deafen members: {}\n",
                            "Embed links: {}\n",
                            "External emojis: {}\n",
                            "Kick members: {}\n",
                            "Manage channels: {}\n",
                            "Manage emojis: {}\n",
                            "Manage guild: {}\n",
                            "Manage messages: {}\n",
                            "Manage nicknames: {}\n",
                            "Manage roles: {}\n",
                            "Manage webhooks: {}\n",
                            "Mention everyone: {}\n",
                            "Move members: {}\n",
                            "Mute members: {}\n",
                            "Read message history: {}\n",
                            "Read messages: {}\n",
                            "Send messages: {}\n",
                            "Send TTS messages: {}\n",
                            "Speak: {}\n",
                            "Use external emojis: {}\n",
                            "Use VAD: {}\n"
                        ),
                        permissions.add_reactions(),
                        permissions.administrator(),
                        permissions.attach_files(),
                        permissions.ban_members(),
                        permissions.change_nickname(),
                        permissions.connect(),
                        permissions.create_invite(),
                        permissions.deafen_members(),
                        permissions.embed_links(),
                        permissions.external_emojis(),
                        permissions.kick_members(),
                        permissions.manage_channels(),
                        permissions.manage_emojis(),
                        permissions.manage_guild(),
                        permissions.manage_messages(),
                        permissions.manage_nicknames(),
                        permissions.manage_roles(),
                        permissions.manage_webhooks(),
                        permissions.mention_everyone(),
                        permissions.move_members(),
                        permissions.mute_members(),
                        permissions.read_message_history(),
                        permissions.read_messages(),
                        permissions.send_messages(),
                        permissions.send_tts_messages(),
                        permissions.speak(),
                        permissions.use_external_emojis(),
                        permissions.use_vad()
                    )
                )
            ));

            if result.is_err() {
                return Err("Failed to send message".to_owned());
            }
        }
    }
});