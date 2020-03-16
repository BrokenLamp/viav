use super::deck;
use lazy_static::lazy_static;
use serenity::model::channel::ChannelType;
use serenity::model::channel::PermissionOverwrite;
use serenity::model::channel::PermissionOverwriteType;
use serenity::model::permissions::Permissions;
use serenity::model::prelude::GuildChannel;
use serenity::model::prelude::GuildId;
use serenity::model::prelude::RoleId;
use serenity::model::prelude::UserId;
use serenity::prelude::Context;
use std::sync::Mutex;

lazy_static! {
    static ref ID: Mutex<u8> = Mutex::new(250);
}

pub fn voice_create(
    ctx: &Context,
    guild_id: GuildId,
    voice_channel: &GuildChannel,
    user_id: UserId,
) -> Option<()> {
    if voice_channel.name == "AFK" {
        return None;
    }

    duplicate_voice_channel(ctx, guild_id, voice_channel)?;

    let id = {
        let mut lock = ID.lock().unwrap();
        *lock = lock.overflowing_add(1).0;
        *lock
    };

    let new_name = format!("{} / {}", voice_channel.name, id);
    let voice_channel_id = voice_channel.id;
    voice_channel_id
        .edit(ctx, |c| c.name::<&str>(new_name.as_ref()))
        .ok()?
        .create_permission(
            ctx,
            &PermissionOverwrite {
                allow: Permissions::MANAGE_CHANNELS | Permissions::MOVE_MEMBERS,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(user_id),
            },
        )
        .ok()?;

    let screen_share_link = format!(
        "https://discordapp.com/channels/{}/{}",
        guild_id.0, voice_channel.id.0
    );

    let channel_type = ChannelType::Text;
    let text_channel = guild_id
        .create_channel(ctx, |c| {
            let mut create_channel = c
                .kind(channel_type)
                .topic(format!(
                    "**Screen Share: {}** - &{}&{}",
                    screen_share_link, voice_channel.id.0, user_id.0
                ))
                .name(format!("voice-viav-{}", id))
                .permissions(vec![
                    // @everyone
                    PermissionOverwrite {
                        allow: Permissions::empty(),
                        deny: Permissions::READ_MESSAGES,
                        kind: PermissionOverwriteType::Role(RoleId::from(guild_id.0)),
                    },
                    // @Viav
                    PermissionOverwrite {
                        allow: Permissions::all(),
                        deny: Permissions::empty(),
                        kind: PermissionOverwriteType::Member(ctx.cache.read().user.id),
                    },
                ]);

            if let Some(category_id) = voice_channel.category_id {
                create_channel = create_channel.category(category_id);
            }
            create_channel
        })
        .ok()?;

    deck::create_deck(ctx, &text_channel, new_name, screen_share_link, user_id)?
        .pin(ctx)
        .ok()?;

    text_channel
        .create_permission(
            ctx,
            &PermissionOverwrite {
                allow: Permissions::READ_MESSAGES,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(user_id),
            },
        )
        .ok()?;

    Some(())
}

fn duplicate_voice_channel(
    ctx: &Context,
    guild_id: GuildId,
    voice_channel: &GuildChannel,
) -> Option<GuildChannel> {
    guild_id
        .create_channel(ctx, |c| {
            let mut create_channel = c
                .kind(ChannelType::Voice)
                .name::<&str>(voice_channel.name.as_ref())
                .permissions(voice_channel.permission_overwrites.clone());

            if let Some(category_id) = voice_channel.category_id {
                create_channel = create_channel.category(category_id);
            }

            if let Some(user_limit) = voice_channel.user_limit {
                create_channel = create_channel.user_limit(user_limit as u32);
            }

            create_channel
        })
        .ok()
}
