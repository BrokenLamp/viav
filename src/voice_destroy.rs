use super::channel_utils;
use serenity::model::prelude::*;
use serenity::prelude::Context;

pub async fn voice_destroy(
    ctx: &Context,
    guild_id: GuildId,
    voice_channel_id: ChannelId,
) -> Option<()> {
    let text_channel = channel_utils::voice_to_text(ctx, guild_id, voice_channel_id).await;
    if let Some(text_channel) = text_channel {
        text_channel.delete(ctx).await.ok()?;
        voice_channel_id.delete(ctx).await.ok()?;
    }
    Some(())
}
