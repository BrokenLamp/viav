use log::trace;
use serenity::{client::Cache, model::prelude::{ChannelId, GuildChannel, GuildId, Permissions, UserId}, prelude::Context};

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

pub async fn voice_to_text(
    ctx: &Context,
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
    trace!("voice to text end not found");
    None
}

pub async fn number_of_connected_users(
    cache: impl AsRef<Cache>,
    guild_id: GuildId,
    voice_channel: ChannelId,
) -> Option<u32> {
    let cache = cache.as_ref();
    let guild = cache
        .guild(guild_id)
        .await?;

    Some(guild
        .voice_states
        .values()
        .fold(0, |acc, v| {
            v.channel_id
                .map(
                    |c| {
                        if c == voice_channel {
                            acc + 1
                        } else {
                            acc
                        }
                    },
                )
                .unwrap_or(acc)
        }))
}

pub struct TopicData {
    pub voice_channel: ChannelId,
    pub owner: UserId,
    pub allow: Permissions,
    pub deny: Permissions,
}

impl TopicData {
    pub fn from_string(data: &str) -> Option<TopicData> {
        let mut split = data.split('&');
        split.next()?;
        Some(TopicData {
            voice_channel: ChannelId(split.next()?.parse::<u64>().ok()?),
            owner: UserId(split.next()?.parse::<u64>().ok()?),
            allow: Permissions::from_bits_truncate(split.next()?.parse::<u64>().ok()?),
            deny: Permissions::from_bits_truncate(split.next()?.parse::<u64>().ok()?),
        })
    }
    pub fn to_string(&self) -> String {
        format!(
            "&{}&{}&{}&{}",
            self.voice_channel.0, self.owner.0, self.allow.bits, self.deny.bits
        )
    }
}
