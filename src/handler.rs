use crate::channel_utils::{text_to_voice, voice_to_text};

use super::deck;
use super::voice_events;
use anyhow::Context;
use async_trait::async_trait;
use log::info;
use serenity::{
    model::{
        channel::{ChannelType, GuildChannel},
        gateway::{Activity, Ready},
        id::{GuildId, UserId},
        prelude::Reaction,
        voice::VoiceState,
    },
    prelude::EventHandler,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: serenity::client::Context, _: Ready) {
        ctx.set_activity(Activity::listening("-viav help")).await;
        info!("Shard {} - online", ctx.shard_id);
    }

    async fn voice_state_update(
        &self,
        ctx: serenity::client::Context,
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
                    let result =
                        voice_events::on_leave(&ctx, guild_id, &guild_channel, old_user_id)
                            .await
                            .context("On Leave Error");
                    if let Err(err) = result {
                        eprint!("{:?}\n\n", err);
                    }
                }
            }
            if let Some(new_id) = new_id {
                let guild_channel = new_id
                    .to_channel(&ctx)
                    .await
                    .ok()
                    .and_then(|channel| channel.guild());
                if let Some(guild_channel) = guild_channel {
                    let result = voice_events::on_join(&ctx, guild_id, &guild_channel, new_user_id)
                        .await
                        .context("On Join Error");
                    if let Err(err) = result {
                        eprint!("{:?}\n\n", err);
                    }
                }
            }
        }
    }

    async fn reaction_add(&self, ctx: serenity::client::Context, reaction: Reaction) {
        if let Some((mut vc, mut tc, owner)) = deck::get_deck_reaction_info(&ctx, &reaction).await {
            let result = deck::on_deck_reaction(&ctx, &reaction, true, &mut vc, &mut tc, owner)
                .await
                .context("Reaction Add Error");
            if let Err(err) = result {
                let result = tc
                    .send_message(ctx, |m| m.content(format!("{:?}", err)))
                    .await
                    .context("Send error message to channel");
                if let Err(err) = result {
                    eprintln!("{:?}", err);
                }
            }
        }
    }

    async fn reaction_remove(&self, ctx: serenity::client::Context, reaction: Reaction) {
        if let Some((mut vc, mut tc, owner)) = deck::get_deck_reaction_info(&ctx, &reaction).await {
            let result = deck::on_deck_reaction(&ctx, &reaction, false, &mut vc, &mut tc, owner)
                .await
                .context("Reaction Remove Error");
            if let Err(err) = result {
                let result = tc
                    .send_message(ctx, |m| m.content(format!("{:?}", err)))
                    .await
                    .context("Send error message to channel");
                if let Err(err) = result {
                    eprintln!("{:?}", err);
                }
            }
        }
    }

    async fn channel_delete(&self, ctx: serenity::client::Context, channel: &GuildChannel) {
        let other_channel = match channel.kind {
            ChannelType::Text => text_to_voice(channel),
            ChannelType::Voice => voice_to_text(&ctx, channel.guild_id, channel.id).await,
            _ => return,
        };
        if let Some(channel) = other_channel {
            let result = channel.delete(&ctx).await.context("Delete channel");
            if let Err(err) = result {
                eprintln!("{:?}", err);
            }
        }
    }
}
