use super::deck;
use crate::channel_utils::voice_to_text;
use crate::models::TopicData;
use anyhow::Context;
use lazy_static::lazy_static;
use log::debug;
use serenity::model::{
    channel::{ChannelType, PermissionOverwrite, PermissionOverwriteType},
    permissions::Permissions,
    prelude::{GuildChannel, GuildId, RoleId, UserId},
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::atomic::{AtomicU8, Ordering},
};
use tokio::time::{sleep, Duration};

lazy_static! {
    static ref ID: AtomicU8 = AtomicU8::new(0);
}

pub async fn voice_create(
    ctx: &serenity::prelude::Context,
    guild_id: GuildId,
    voice_channel: &GuildChannel,
    user_id: UserId,
) -> anyhow::Result<()> {
    if let Some(guild) = guild_id.to_guild_cached(ctx).await {
        if Some(voice_channel.id) == guild.afk_channel_id {
            return Ok(());
        }
    }
    debug!("out of guild_cached");

    if voice_channel.name.starts_with("&") {
        return Ok(());
    }

    if let Some(tc) = voice_to_text(ctx, guild_id, voice_channel.id).await {
        tc.send_message(ctx, |m| {
            m.content(
                "Tried to create new voice channel but it already exists. Using this text channel.",
            )
        })
        .await?;
        return Ok(());
    }

    let id = ID.fetch_add(1, Ordering::Relaxed);
    let mut hasher = DefaultHasher::new();
    id.hash(&mut hasher);
    let mut id = format!("{:x}", hasher.finish());
    id.truncate(4);

    let viav_user_id = ctx.cache.current_user().await.id;

    let old_name = voice_channel.name.clone();
    let new_name = format!("{} / {}", old_name, id);
    let voice_channel_id = voice_channel.id;
    debug!("create new channel");
    let mut new_channel = voice_channel_id
        .edit(ctx, |c| c.name::<&str>(new_name.as_ref()))
        .await
        .with_context(|| format!("Editting channel: {}", voice_channel_id))?;

    debug!("get default perms");
    let default_perms = (&voice_channel.permission_overwrites)
        .into_iter()
        .find(|p| p.kind == PermissionOverwriteType::Role(RoleId(guild_id.0)))
        .map(|p| (p.allow, p.deny))
        .unwrap_or((Permissions::READ_MESSAGES, Permissions::empty()));

    debug!("construct topic data");
    let topic_data = TopicData {
        voice_channel: voice_channel_id,
        owner: user_id,
        allow: default_perms.0,
        deny: default_perms.1,
    };

    debug!("create text channel");
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
            kind: PermissionOverwriteType::Member(viav_user_id),
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
        .context("Create text channel")?;

    debug!("set channel perms");
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
        .context("Add permissions to text channel")?;

    let deck_future = async {
        debug!("create deck");
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
            .context("Send controls")?;
        deck::create_deck(ctx, text_channel.id, new_name, user_id)
            .await
            .context("Create deck")?
            .pin(ctx)
            .await
            .context("Pin deck")?;
        Ok::<(), anyhow::Error>(())
    };

    let voice_duplicate_future = async {
        debug!("pausing.");
        sleep(Duration::from_millis(1500)).await;
        debug!("unpaused.");

        duplicate_voice_channel(ctx, guild_id, voice_channel, &old_name).await
    };

    let (_, permission) = futures::join!(deck_future, voice_duplicate_future);

    if let Err(err) = permission {
        new_channel.edit(ctx, |c| c.name(&old_name)).await?;
        return Err(err);
    }

    new_channel
        .create_permission(
            ctx,
            &PermissionOverwrite {
                allow: Permissions::all(),
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(viav_user_id),
            },
        )
        .await?;
    new_channel
        .create_permission(
            ctx,
            &PermissionOverwrite {
                allow: Permissions::MANAGE_CHANNELS | Permissions::MOVE_MEMBERS,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(user_id),
            },
        )
        .await?;

    debug!("voice_create end");

    Ok(())
}

async fn duplicate_voice_channel<S: ToString>(
    ctx: &serenity::prelude::Context,
    guild_id: GuildId,
    voice_channel: &GuildChannel,
    name: S,
) -> anyhow::Result<GuildChannel> {
    debug!("duplicate_voice_channel start");
    let channel = guild_id
        .create_channel(ctx, |c| {
            let mut create_channel = c
                .kind(ChannelType::Voice)
                .name(name.to_string())
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
        .await?;
    debug!("duplicate_voice_channel end");
    Ok(channel)
}
