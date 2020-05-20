use super::deck;
use crate::channel_utils::TopicData;
use lazy_static::lazy_static;
use log::trace;
use serenity::model::channel::ChannelType;
use serenity::model::channel::PermissionOverwrite;
use serenity::model::channel::PermissionOverwriteType;
use serenity::model::permissions::Permissions;
use serenity::model::prelude::GuildChannel;
use serenity::model::prelude::GuildId;
use serenity::model::prelude::RoleId;
use serenity::model::prelude::UserId;
use serenity::prelude::Context;
use std::sync::atomic::{AtomicU8, Ordering};
use tokio::time::{delay_for, Duration};

lazy_static! {
    static ref ID: AtomicU8 = AtomicU8::new(0);
}

pub async fn voice_create(
    ctx: &Context,
    guild_id: GuildId,
    voice_channel: &GuildChannel,
    user_id: UserId,
) -> Option<()> {
    trace!("voice_create start");
    if let Some(guild) = guild_id.to_guild_cached(ctx).await {
        if Some(voice_channel.id) == guild.read().await.afk_channel_id {
            return None;
        }
    }
    trace!("out of guild_cached");

    if voice_channel.name.starts_with("&") {
        return None;
    }

    let id = ID.fetch_add(1, Ordering::Relaxed);

    let old_name = voice_channel.name.clone();
    let new_name = format!("{} / {}", old_name, id);
    let voice_channel_id = voice_channel.id;
    let new_channel = voice_channel_id
        .edit(ctx, |c| c.name::<&str>(new_name.as_ref()))
        .await
        .ok()?;

    trace!("get default perms");
    let default_perms = (&voice_channel.permission_overwrites)
        .into_iter()
        .find(|p| p.kind == PermissionOverwriteType::Role(RoleId(guild_id.0)))
        .map(|p| (p.allow, p.deny))
        .unwrap_or((Permissions::READ_MESSAGES, Permissions::empty()));

    trace!("construct topic data");
    let topic_data = TopicData {
        voice_channel: voice_channel_id,
        owner: user_id,
        allow: default_perms.0,
        deny: default_perms.1,
    };

    trace!("create text channel");
    let channel_type = ChannelType::Text;
    let permissions = vec![
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
            kind: PermissionOverwriteType::Member(ctx.cache.read().await.user.id),
        },
    ];
    let text_channel = guild_id
        .create_channel(ctx, |c| {
            let mut create_channel = c
                .kind(channel_type)
                .topic(format!("Viav - {}", topic_data.to_string()))
                .name(format!("voice-viav-{}", id))
                .permissions(permissions);

            if let Some(category_id) = voice_channel.category_id {
                create_channel = create_channel.category(category_id);
            }
            create_channel
        })
        .await
        .ok()?;

    trace!("set channel perms");
    text_channel
        .create_permission(
            ctx,
            &PermissionOverwrite {
                allow: Permissions::READ_MESSAGES,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(user_id),
            },
        )
        .await
        .ok()?;

    let deck_future = async {
        trace!("create deck");
        text_channel
            .id
            .send_message(&ctx, |c| {
                c.embed(|e| {
                    super::help::create_help_embed(e).field(
                        "Controls",
                        "<:lock:684471911920566281> - Lock voice channel\n
                        <:eye:684471928739725376> - Hide voice channel\n
                        <:alert:684470685430448128> - Mark text channel NSFW\n",
                        false,
                    )
                })
            })
            .await
            .ok()?;
        deck::create_deck(ctx, text_channel.id, new_name, user_id)
            .await?
            .pin(ctx)
            .await
            .ok()
    };

    let permission_future = async {
        trace!("pausing.");
        delay_for(Duration::from_millis(1500)).await;
        trace!("unpaused.");

        duplicate_voice_channel(ctx, guild_id, voice_channel, old_name).await
    };

    futures::join!(deck_future, permission_future);

    new_channel
        .create_permission(
            ctx,
            &PermissionOverwrite {
                allow: Permissions::MANAGE_CHANNELS | Permissions::MOVE_MEMBERS,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(user_id),
            },
        )
        .await
        .ok()?;

    trace!("voice_create end");

    Some(())
}

async fn duplicate_voice_channel(
    ctx: &Context,
    guild_id: GuildId,
    voice_channel: &GuildChannel,
    name: String,
) -> Option<GuildChannel> {
    trace!("duplicate_voice_channel start");
    let channel = guild_id
        .create_channel(ctx, |c| {
            let mut create_channel = c
                .kind(ChannelType::Voice)
                .name(name)
                .permissions(voice_channel.permission_overwrites.clone())
                .bitrate(voice_channel.bitrate.unwrap_or(64) as u32);

            if let Some(category_id) = voice_channel.category_id {
                create_channel = create_channel.category(category_id);
            }

            if let Some(user_limit) = voice_channel.user_limit {
                create_channel = create_channel.user_limit(user_limit as u32);
            }

            create_channel
        })
        .await
        .ok();
    trace!("duplicate_voice_channel end");
    channel
}
