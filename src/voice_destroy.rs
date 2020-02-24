use serenity::model::prelude::GuildChannel;
use serenity::model::prelude::GuildId;
use serenity::prelude::Context;
use serenity::prelude::RwLock;
use std::sync::Arc;

pub fn voice_destroy(
    ctx: &mut Context,
    _guild_id: GuildId,
    voice_channel: Arc<RwLock<GuildChannel>>,
) -> Option<()> {
    voice_channel.read().delete(&ctx).ok()?;

    Some(())
}
