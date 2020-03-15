use super::channel_utils;
use serenity::model::prelude::GuildChannel;
use serenity::model::prelude::GuildId;
use serenity::prelude::Context;

pub fn voice_destroy(ctx: &Context, guild_id: GuildId, voice_channel: &GuildChannel) -> Option<()> {
    channel_utils::voice_to_text(ctx, guild_id, voice_channel.id).map(|text_channel| {
        text_channel.delete(ctx).ok();
        voice_channel.delete(ctx).ok();
    })
}
