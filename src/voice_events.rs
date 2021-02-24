use crate::channel_utils::number_of_connected_users;

use super::{channel_utils, voice_create, voice_destroy};
use anyhow::Context;
use serenity::{
    model::{
        channel::{GuildChannel, PermissionOverwrite, PermissionOverwriteType},
        id::GuildId,
        permissions::Permissions,
        prelude::*,
    },
    utils::Colour,
};

pub async fn on_join(
    ctx: &serenity::prelude::Context,
    guild_id: GuildId,
    voice_channel: &GuildChannel,
    user_id: UserId,
) -> anyhow::Result<()> {
    let num_members = number_of_connected_users(ctx, guild_id, voice_channel.id)
        .await
        .ok_or(anyhow::anyhow!("Could not get number of connected users"))?;

    if num_members == 1 {
        voice_create::voice_create(ctx, guild_id, voice_channel, user_id).await?;
    } else {
        if let Some(text_channel) =
            channel_utils::voice_to_text(ctx, guild_id, voice_channel.id).await
        {
            send_join_leave_message(ctx, text_channel, user_id, guild_id, "joined").await?;
            let result = text_channel
                .create_permission(
                    ctx,
                    &PermissionOverwrite {
                        allow: Permissions::READ_MESSAGES,
                        deny: Permissions::empty(),
                        kind: PermissionOverwriteType::Member(user_id),
                    },
                )
                .await;
            if let Err(err) = result {
                text_channel
                    .send_message(ctx, |c| {
                        c.embed(|e| {
                            e.colour(Colour::from_rgb(200, 50, 80))
                                .description(format!("{:?}", err))
                        })
                    })
                    .await
                    .context("Join / leave message")?;
                anyhow::bail!(err);
            }
        }
    }

    Ok(())
}

pub async fn on_leave(
    ctx: &serenity::prelude::Context,
    guild_id: GuildId,
    voice_channel: &GuildChannel,
    user_id: UserId,
) -> anyhow::Result<()> {
    let num_members = number_of_connected_users(ctx, guild_id, voice_channel.id)
        .await
        .ok_or(anyhow::anyhow!(
            "Could not get number of connected users for voice channel: {}",
            voice_channel.id
        ))?;

    if num_members == 0 {
        voice_destroy::voice_destroy(ctx, guild_id, voice_channel.id).await?;
    } else {
        if let Some(text_channel) =
            channel_utils::voice_to_text(ctx, guild_id, voice_channel.id).await
        {
            text_channel
                .delete_permission(ctx, PermissionOverwriteType::Member(user_id))
                .await
                .ok();
            send_join_leave_message(ctx, text_channel, user_id, guild_id, "left").await?;
        }
    }

    Ok(())
}

async fn send_join_leave_message(
    ctx: &serenity::prelude::Context,
    text_channel: ChannelId,
    user_id: UserId,
    guild_id: GuildId,
    message: &str,
) -> anyhow::Result<Message> {
    let user = user_id.to_user(ctx).await?;
    let username = user
        .nick_in(ctx, guild_id)
        .await
        .unwrap_or(user.name.clone());
    Ok(text_channel
        .send_message(ctx, |c| {
            c.embed(|e| {
                e.author(|a| {
                    a.name(format!("{} - {}", username, message))
                        .icon_url(user.face())
                })
                .colour(Colour::from_rgb(103, 58, 183))
            })
        })
        .await
        .context("Join / leave message")?)
}
