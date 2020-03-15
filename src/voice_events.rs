use super::{channel_utils, voice_create, voice_destroy};
use serenity::model::channel::PermissionOverwrite;
use serenity::model::channel::PermissionOverwriteType;
use serenity::model::permissions::Permissions;
use serenity::{
    model::{channel::GuildChannel, id::GuildId, prelude::UserId},
    prelude::Context,
};

pub fn on_join(
    ctx: &Context,
    guild_id: GuildId,
    voice_channel: &GuildChannel,
    user_id: UserId,
) -> Option<()> {
    let num_members = voice_channel.members(ctx).ok()?.len();

    if num_members == 1 {
        voice_create::voice_create(ctx, guild_id, voice_channel, user_id)?;
    } else {
        channel_utils::voice_to_text(ctx, guild_id, voice_channel.id).map(|text_channel| {
            text_channel
                .create_permission(
                    ctx,
                    &PermissionOverwrite {
                        allow: Permissions::READ_MESSAGES,
                        deny: Permissions::empty(),
                        kind: PermissionOverwriteType::Member(user_id),
                    },
                )
                .ok();
        })?;
    }

    Some(())
}

pub fn on_leave(
    ctx: &Context,
    guild_id: GuildId,
    voice_channel: &GuildChannel,
    user_id: UserId,
) -> Option<()> {
    let num_members = voice_channel.members(ctx).ok()?.len();

    if num_members == 0 {
        voice_destroy::voice_destroy(ctx, guild_id, voice_channel);
    } else {
        channel_utils::voice_to_text(ctx, guild_id, voice_channel.id).map(|text_channel| {
            text_channel
                .delete_permission(ctx, PermissionOverwriteType::Member(user_id))
                .ok();
        })?;
    }

    Some(())
}
