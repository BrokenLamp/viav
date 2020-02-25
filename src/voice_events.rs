use super::voice_create;
use super::voice_destroy;
use serenity::{
    model::{channel::GuildChannel, id::GuildId, prelude::UserId},
    prelude::{Context, RwLock},
};
use std::sync::Arc;

pub fn on_join(
    ctx: &mut Context,
    guild_id: GuildId,
    voice_channel: Arc<RwLock<GuildChannel>>,
    user_id: UserId,
) -> Option<()> {
    if voice_channel.read().members(&ctx).ok()?.len() == 1 {
        voice_create::voice_create(ctx, guild_id, voice_channel, user_id)?;
    }

    Some(())
}

pub fn on_leave(
    ctx: &mut Context,
    guild_id: GuildId,
    voice_channel: Arc<RwLock<GuildChannel>>,
    _user_id: UserId,
) -> Option<()> {
    if voice_channel.read().members(&ctx).ok()?.len() == 0 {
        voice_destroy::voice_destroy(ctx, guild_id, voice_channel);
    }
    Some(())
}
