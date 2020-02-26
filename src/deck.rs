use serenity::model::prelude::GuildChannel;
use serenity::model::prelude::GuildId;
use serenity::prelude::Context;
use serenity::prelude::RwLock;
use std::sync::Arc;

pub fn on_deck_reaction_add(
    ctx: &mut Context,
    guild_id: GuildId,
    text_channel: Arc<RwLock<GuildChannel>>,
) -> Option<()> {
    Some(())
}

pub fn on_deck_reaction_remove(
    ctx: &mut Context,
    guild_id: GuildId,
    text_channel: Arc<RwLock<GuildChannel>>,
) -> Option<()> {
    Some(())
}
