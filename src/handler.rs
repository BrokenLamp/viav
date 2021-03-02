use super::deck;
use super::voice_events;
use crate::error_display::HandleError;
use crate::{
    channel_utils::{text_to_voice, voice_to_text},
    slash_commands,
};
use anyhow::Context;
use async_trait::async_trait;
use log::info;
use serenity::{
    model::{
        channel::{ChannelType, GuildChannel},
        gateway::{Activity, Ready},
        id::{GuildId, UserId},
        interactions::Interaction,
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
        slash_commands::register_slash_commands(&ctx)
            .await
            .expect("Register slash commands");
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
                            .context(format!(
                                "On Leave Error in {} : {}",
                                guild_channel.guild_id,
                                guild_channel.name()
                            ));
                    result.handle(&ctx, None).await;
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
                        .context(format!(
                            "On Join Error in {} : {}",
                            guild_channel.guild_id,
                            guild_channel.name()
                        ));
                    result.handle(&ctx, None).await;
                }
            }
        }
    }

    async fn reaction_add(&self, ctx: serenity::client::Context, reaction: Reaction) {
        deck::on_deck_reaction(&ctx, &reaction, true)
            .await
            .handle(&ctx, None)
            .await;
    }

    async fn reaction_remove(&self, ctx: serenity::client::Context, reaction: Reaction) {
        deck::on_deck_reaction(&ctx, &reaction, false)
            .await
            .handle(&ctx, None)
            .await;
    }

    async fn channel_delete(&self, ctx: serenity::client::Context, channel: &GuildChannel) {
        let other_channel = match channel.kind {
            ChannelType::Text => text_to_voice(channel),
            ChannelType::Voice => voice_to_text(&ctx, channel.guild_id, channel.id).await,
            _ => return,
        };
        if let Some(channel) = other_channel {
            channel
                .delete(&ctx)
                .await
                .context("Delete channel")
                .handle(&ctx, None)
                .await;
        }
    }

    async fn interaction_create(&self, ctx: serenity::client::Context, interaction: Interaction) {
        slash_commands::handle_slash_command(&ctx, interaction)
            .await
            .handle(&ctx, None)
            .await;
    }
}
