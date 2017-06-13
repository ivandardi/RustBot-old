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

            macro_rules! format_permissions {
                ($perms:ident, $($text:expr => $method:ident),+) => {
                    format!(
                        concat!(
                            $(
                                concat!($text, ": {}\n")
                            ),+
                        ),
                        $(
                            $perms.$method()
                        ),+
                    )
                }
            }
            
            let result = msg.channel_id.send_message(|cm| cm.embed(|ce|
                ce.title("Permissions")
                .color(Colour::blue())
                .description(
                    &format_permissions!(permissions,
                        "Add reactions" => add_reactions,
                        "Administrator" => administrator,
                        "Ban members" => ban_members,
                        "Change nickname" => change_nickname,
                        "Connect" => connect,
                        "Create invite" => create_invite,
                        "Deafen members" => deafen_members,
                        "Embed links" => embed_links,
                        "External emojis" => external_emojis,
                        "Kick members" => kick_members,
                        "Manage channels" => manage_channels,
                        "Manage emojis" => manage_emojis,
                        "Manage guild" => manage_guild,
                        "Manage messages" => manage_messages,
                        "Manage nicknames" => manage_nicknames,
                        "Manage roles" => manage_roles,
                        "Manage webhooks" => manage_webhooks,
                        "Mention everyone" => mention_everyone,
                        "Move members" => move_members,
                        "Mute members" => mute_members,
                        "Read message history" => read_message_history,
                        "Read messages" => read_messages,
                        "Send messages" => send_messages,
                        "Send TTS messages" => send_tts_messages,
                        "Speak" => speak,
                        "Use external emojis" => use_external_emojis,
                        "Use VAD" => use_vad
                    )
                )
            ));

            if result.is_err() {
                return Err("Failed to send message".to_owned());
            }
        }
    }
});