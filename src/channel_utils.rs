use crate::models::TopicData;
use crate::MASTER_USER;
use log::debug;
use serenity::{
    client::Cache,
    model::{
        id::UserId,
        prelude::{ChannelId, GuildChannel, GuildId},
    },
};

/// Given a text channel, returns the corresponding voice channel ID
#[allow(dead_code)]
pub fn text_to_voice(channel: &GuildChannel) -> Option<ChannelId> {
    let topic = channel.topic.clone()?;
    let mut split = topic.split("&");
    split.next()?;
    let vc_id = split.next()?.parse::<u64>().ok()?;
    if vc_id != 0 {
        Some(ChannelId(vc_id))
    } else {
        None
    }
}

/// Given a voice channel ID returns the corresponding text channel ID
pub async fn voice_to_text(
    ctx: &serenity::client::Context,
    guild_id: GuildId,
    voice_channel: ChannelId,
) -> Option<ChannelId> {
    for (channel_id, guild_channel) in guild_id.channels(ctx).await.ok()? {
        if let Some(topic) = guild_channel.topic {
            let mut split = topic.split("&");
            split.next();
            if let Some(topic_id) = split.next() {
                if let Ok(topic_id) = topic_id.parse::<u64>() {
                    if topic_id == voice_channel.0 {
                        return Some(channel_id);
                    }
                }
            }
        }
    }
    debug!("voice to text end not found");
    None
}

pub async fn number_of_connected_users(
    cache: impl AsRef<Cache>,
    guild_id: GuildId,
    voice_channel: ChannelId,
) -> Option<u32> {
    let cache = cache.as_ref();
    let guild = cache.guild(guild_id).await?;

    Some(guild.voice_states.values().fold(0, |acc, v| {
        v.channel_id
            .map(|c| if c == voice_channel { acc + 1 } else { acc })
            .unwrap_or(acc)
    }))
}

/// Checks if a user is allowed to perform tasks in a channel
pub async fn is_user_perms(
    ctx: &serenity::client::Context,
    user_id: UserId,
    topic: &TopicData,
    text_channel: &GuildChannel,
) -> bool {
    if topic.owner == user_id || MASTER_USER == user_id {
        return true;
    }
    text_channel
        .permissions_for_user(ctx, user_id)
        .await
        .map(|p| p.manage_channels())
        .unwrap_or(false)
}
