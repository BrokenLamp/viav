use log::trace;
use serenity::model::prelude::{ChannelId, GuildChannel, GuildId};
use serenity::prelude::Context;

#[allow(dead_code)]
pub fn text_to_voice(channel: GuildChannel) -> Option<ChannelId> {
    let topic = channel.topic?;
    let mut split = topic.split("&");
    split.next()?;
    let vc_id = split.next()?.parse::<u64>().ok()?;
    if vc_id != 0 {
        Some(ChannelId(vc_id))
    } else {
        None
    }
}

pub fn voice_to_text(
    ctx: &Context,
    guild_id: GuildId,
    voice_channel: ChannelId,
) -> Option<ChannelId> {
    trace!("voice to text start");
    for (channel_id, guild_channel) in guild_id.channels(&ctx).ok()? {
        if let Some(topic) = guild_channel.topic {
            trace!("has topic");
            let mut split = topic.split("&");
            split.next();
            if let Some(topic_id) = split.next() {
                trace!("has id");
                if let Ok(topic_id) = topic_id.parse::<u64>() {
                    trace!("id is u64");
                    if topic_id == voice_channel.0 {
                        trace!("voice to text end found");
                        return Some(channel_id);
                    }
                }
            }
        }
    }
    trace!("voice to text end not found");
    None
}
