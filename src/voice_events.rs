use serenity::prelude::RwLock;
use serenity::{
    model::{
        channel::{ChannelType, GuildChannel},
        id::GuildId,
    },
    prelude::Context,
};
use std::sync::Arc;

pub fn on_join(
    ctx: &mut Context,
    guild_id: GuildId,
    voice_channel: Arc<RwLock<GuildChannel>>,
) -> Option<()> {
    println!("JOIN");
    if voice_channel.read().members(&ctx).ok()?.len() != 1 {
        return None;
    }
    guild_id
        .create_channel(ctx, |c| {
            let mut create_channel = c
                .kind(ChannelType::Voice)
                .name(voice_channel.read().name.clone())
                .position((voice_channel.read().position - 1) as u32)
                .permissions(voice_channel.read().permission_overwrites.clone());

            if let Some(category_id) = voice_channel.read().category_id {
                create_channel = create_channel.category(category_id);
            }

            if let Some(user_limit) = voice_channel.read().user_limit {
                create_channel = create_channel.user_limit(user_limit as u32);
            }

            create_channel
        })
        .ok()?;
    Some(())
}

pub fn on_leave(
    ctx: &mut Context,
    _guild_id: GuildId,
    voice_channel: Arc<RwLock<GuildChannel>>,
) -> Option<()> {
    println!("LEAVE");
    if voice_channel.read().members(&ctx).ok()?.len() == 0 {
        voice_channel.read().delete(&ctx).ok()?;
    }
    Some(())
}
