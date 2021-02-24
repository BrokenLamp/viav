use super::channel_utils;
use anyhow::Context;
use serenity::model::prelude::*;

pub async fn voice_destroy(
    ctx: &serenity::prelude::Context,
    guild_id: GuildId,
    voice_channel_id: ChannelId,
) -> anyhow::Result<()> {
    let text_channel = channel_utils::voice_to_text(ctx, guild_id, voice_channel_id)
        .await
        .with_context(|| {
            format!(
                "Could not find text channel for voice channel: {}",
                voice_channel_id
            )
        })?;
    text_channel
        .delete(ctx)
        .await
        .context("delete text channel")?;
    // Voice channel will be automatically deleted by another hook
    Ok(())
}
