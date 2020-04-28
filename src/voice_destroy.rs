use super::channel_utils;
use serenity::model::prelude::*;
use serenity::prelude::Context;

pub fn voice_destroy(ctx: &Context, guild_id: GuildId, voice_channel_id: ChannelId) -> Option<()> {
    channel_utils::voice_to_text(ctx, guild_id, voice_channel_id).map(|text_channel| {
        text_channel.delete(ctx).ok();
        voice_channel_id.delete(ctx).ok();
    })
}
