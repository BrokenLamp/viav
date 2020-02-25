use serenity::model::prelude::GuildChannel;
use serenity::model::prelude::GuildId;
use serenity::prelude::Context;
use serenity::prelude::RwLock;
use std::sync::Arc;

pub fn voice_destroy(
    ctx: &mut Context,
    guild_id: GuildId,
    voice_channel: Arc<RwLock<GuildChannel>>,
) -> Option<()> {
    let id = voice_channel.read().id.0;

    voice_channel.read().delete(&ctx).ok()?;

    let channels = guild_id.channels(&ctx).ok()?;
    for (channel_id, guild_channel) in channels {
        if let Some(topic) = guild_channel.topic {
            let mut split = topic.split("&");
            split.next();
            if let Some(topic_id) = split.next() {
                if let Ok(topic_id) = topic_id.parse::<u64>() {
                    if topic_id == id {
                        channel_id.delete(&ctx).ok()?;
                    }
                }
            }
        }
    }

    Some(())
}
