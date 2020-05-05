use super::deck;
use super::voice_events;
use async_trait::async_trait;
use log::info;
use serenity::model::prelude::Reaction;
use serenity::{
    model::{
        gateway::{Activity, Ready},
        id::{GuildId, UserId},
        voice::VoiceState,
    },
    prelude::{Context, EventHandler},
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        ctx.set_activity(Activity::listening("-viav help")).await;
        info!("Shard {} - online", ctx.shard_id);
    }

    async fn voice_state_update(
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
                let guild_channel = old_id
                    .to_channel(&ctx)
                    .await
                    .ok()
                    .and_then(|channel| channel.guild());
                if let Some(guild_channel) = guild_channel {
                    voice_events::on_leave(&ctx, guild_id, &guild_channel, old_user_id).await;
                }
            }
            if let Some(new_id) = new_id {
                let guild_channel = new_id
                    .to_channel(&ctx)
                    .await
                    .ok()
                    .and_then(|channel| channel.guild());
                if let Some(guild_channel) = guild_channel {
                    voice_events::on_join(&ctx, guild_id, &guild_channel, new_user_id).await;
                }
            }
        }
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        if let Some((mut vc, mut tc, owner)) = deck::get_deck_reaction_info(&ctx, &reaction).await {
            deck::on_deck_reaction(&ctx, &reaction, true, &mut vc, &mut tc, owner).await;
        }
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        if let Some((mut vc, mut tc, owner)) = deck::get_deck_reaction_info(&ctx, &reaction).await {
            deck::on_deck_reaction(&ctx, &reaction, false, &mut vc, &mut tc, owner).await;
        }
    }
}
