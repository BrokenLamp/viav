use super::{channel_utils, voice_create, voice_destroy};
use log::trace;
use serenity::{
    model::{
        channel::{GuildChannel, PermissionOverwrite, PermissionOverwriteType},
        id::GuildId,
        permissions::Permissions,
        prelude::*,
    },
    prelude::*,
    utils::Colour,
};

pub async fn on_join(
    ctx: &Context,
    guild_id: GuildId,
    voice_channel: &GuildChannel,
    user_id: UserId,
) -> Option<()> {
    let num_members = voice_channel.members(ctx).await.ok()?.len();
    trace!("{}", num_members);

    if num_members == 1 {
        trace!("num_members == 1");
        voice_create::voice_create(ctx, guild_id, voice_channel, user_id).await?;
    } else {
        trace!("num_members != 1");
        if let Some(text_channel) =
            channel_utils::voice_to_text(ctx, guild_id, voice_channel.id).await
        {
            send_join_leave_message(ctx, text_channel, user_id, guild_id, "joined").await;
            trace!("create permission start");
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
                .ok();
            trace!("create permission end");
        }
    }

    trace!("on_join end");

    Some(())
}

pub async fn on_leave(
    ctx: &Context,
    guild_id: GuildId,
    voice_channel: &GuildChannel,
    user_id: UserId,
) -> Option<()> {
    let num_members = voice_channel.members(ctx).await.ok()?.len();
    trace!("{}", num_members);

    if num_members == 0 {
        voice_destroy::voice_destroy(ctx, guild_id, voice_channel.id).await;
    } else {
        if let Some(text_channel) =
            channel_utils::voice_to_text(ctx, guild_id, voice_channel.id).await
        {
            text_channel
                .delete_permission(ctx, PermissionOverwriteType::Member(user_id))
                .await
                .ok();
            send_join_leave_message(ctx, text_channel, user_id, guild_id, "left").await;
        }
    }

    Some(())
}

async fn send_join_leave_message(
    ctx: &Context,
    text_channel: ChannelId,
    user_id: UserId,
    guild_id: GuildId,
    message: &str,
) -> Option<Message> {
    let user = user_id.to_user(ctx).await.ok()?;
    let username = user
        .nick_in(ctx, guild_id)
        .await
        .unwrap_or(user.name.clone());
    text_channel
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
        .ok()
}
