use crate::{channel_utils::is_user_perms, help, models::TopicData};
use anyhow::Context;
use serenity::model::{
    channel::{PermissionOverwrite, PermissionOverwriteType},
    id::UserId,
    prelude::{GuildChannel, RoleId},
    Permissions,
};
use std::convert::TryFrom;

pub enum Operation {
    Lock(bool),
    Hide(bool),
    NSFW(bool),
    Help,
}

impl Operation {
    pub async fn apply(
        &self,
        ctx: &serenity::client::Context,
        user_id: UserId,
        text_channel: &GuildChannel,
    ) -> anyhow::Result<Option<String>> {
        let topic = TopicData::try_from(text_channel);
        let out: Option<String> = match self {
            Operation::Lock(is_locked) => {
                let topic = topic?;
                if !is_user_perms(ctx, user_id, &topic, text_channel).await {
                    return Ok(None);
                }
                let mut voice_channel = topic.voice_channel(ctx).await?;

                voice_channel
                    .edit(ctx, |e| e.user_limit(*is_locked as u64))
                    .await
                    .context("Failed to lock voice channel")?;
                Some(String::from(if *is_locked { "Locked" } else { "Unlocked" }))
            }

            Operation::Hide(is_hidden) => {
                let topic = topic?;
                if !is_user_perms(ctx, user_id, &topic, text_channel).await {
                    return Ok(None);
                }
                let voice_channel = topic.voice_channel(ctx).await?;
                let permissions = if *is_hidden {
                    PermissionOverwrite {
                        allow: Permissions::empty(),
                        deny: Permissions::READ_MESSAGES | Permissions::CONNECT,
                        kind: PermissionOverwriteType::Role(RoleId(voice_channel.guild_id.0)),
                    }
                } else {
                    PermissionOverwrite {
                        allow: topic.allow,
                        deny: topic.deny,
                        kind: PermissionOverwriteType::Role(RoleId(voice_channel.guild_id.0)),
                    }
                };
                voice_channel
                    .create_permission(ctx, &permissions)
                    .await
                    .context("Failed to hide voice channel")?;
                Some(String::from(if *is_hidden { "Hidden" } else { "Unhidden" }))
            }

            Operation::NSFW(is_enable) => {
                let topic = topic?;
                if !is_user_perms(ctx, user_id, &topic, text_channel).await {
                    return Ok(None);
                }
                text_channel
                    .id
                    .edit(ctx, |e| e.nsfw(*is_enable))
                    .await
                    .context("Failed to set text channel to NSFW")?;
                Some(String::from(if *is_enable {
                    "Set nsfw"
                } else {
                    "Unset NSFW"
                }))
            }

            Operation::Help => {
                help::send_help(ctx, text_channel.id).await;
                None
            }
        };
        Ok(out)
    }
}
