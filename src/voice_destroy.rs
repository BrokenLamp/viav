use serenity::model::prelude::GuildChannel;
use serenity::model::prelude::GuildId;
use serenity::prelude::Context;
use serenity::prelude::RwLock;
use std::sync::Arc;

pub fn voice_destroy(
    ctx: &Context,
    guild_id: GuildId,
    voice_channel: Arc<RwLock<GuildChannel>>,
) -> Option<()> {
    let voice_channel = voice_channel.read();

    let mut is_viav_channel = false;

    'search: for (channel_id, guild_channel) in guild_id.channels(&ctx).ok()? {
        if let Some(topic) = guild_channel.topic {
            let mut split = topic.split("&");
            split.next();
            if let Some(topic_id) = split.next() {
                if let Ok(topic_id) = topic_id.parse::<u64>() {
                    if topic_id == voice_channel.id.0 {
                        channel_id.delete(ctx).ok()?;
                        is_viav_channel = true;
                        break 'search;
                    }
                }
            }
        }
    }

    if is_viav_channel {
        voice_channel.delete(ctx).ok()?;
        Some(())
    } else {
        None
    }
}
