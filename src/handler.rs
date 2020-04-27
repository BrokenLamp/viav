use serenity::model::prelude::Reaction;
use serenity::{
    model::{
        gateway::{Activity, Ready},
        id::{GuildId, UserId},
        voice::VoiceState,
    },
    prelude::{Context, EventHandler},
};

use super::deck;
use super::voice_events;

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, _: Ready) {
        ctx.set_activity(Activity::listening("-viav help"));
    }

    fn voice_state_update(
        &self,
        ctx: Context,
        guild: Option<GuildId>,
        old: Option<VoiceState>,
        new: VoiceState,
    ) {
        let guild_id = match guild {
            Some(guild_id) => guild_id,
            None => return,
        };

        let (new_id, new_user_id) = (new.channel_id, new.user_id);
        let (old_id, old_user_id) = match &old {
            Some(old) => (old.channel_id, old.user_id),
            None => (None, UserId(0)),
        };

        if new_id != old_id {
            if let Some(old_id) = old_id {
                old_id
                    .to_channel(&ctx)
                    .ok()
                    .and_then(|channel| channel.guild())
                    .and_then(|channel| {
                        voice_events::on_leave(&ctx, guild_id, &*channel.read(), old_user_id)
                    });
            }
            if let Some(new_id) = new_id {
                new_id
                    .to_channel(&ctx)
                    .ok()
                    .and_then(|channel| channel.guild())
                    .and_then(|channel| {
                        voice_events::on_join(&ctx, guild_id, &*channel.read(), new_user_id)
                    });
            }
        }
    }

    fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        if let Some((mut vc, mut tc, owner)) = deck::get_deck_reaction_info(&ctx, &reaction) {
            deck::on_deck_reaction(&ctx, &reaction, true, &mut vc, &mut tc, owner);
        }
    }

    fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        if let Some((mut vc, mut tc, owner)) = deck::get_deck_reaction_info(&ctx, &reaction) {
            deck::on_deck_reaction(&ctx, &reaction, false, &mut vc, &mut tc, owner);
        }
    }
}
