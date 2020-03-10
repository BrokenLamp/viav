use maplit::hashmap;
use serenity::{
    model::prelude::{GuildChannel, Reaction, User},
    prelude::Context,
};
use std::collections::HashMap;

trait ReactionCommand {
    fn on_add(
        &self,
        ctx: &Context,
        reaction: &Reaction,
        voice_channel: &mut GuildChannel,
        text_channel: &mut GuildChannel,
        _owner: User,
    ) -> Option<()> {
        None
    }
    fn on_remove(
        &self,
        ctx: &Context,
        reaction: &Reaction,
        voice_channel: &mut GuildChannel,
        text_channel: &mut GuildChannel,
        _owner: User,
    ) -> Option<()> {
        None
    }
}

struct Lock;
impl ReactionCommand for Lock {}

pub const ACTIONS: HashMap<u64, Box<dyn ReactionCommand>> = hashmap! {
    684471911920566281u64 => Box::new(Lock) as Box<dyn ReactionCommand>,
};
