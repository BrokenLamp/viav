use serenity::{
    model::id::{ChannelId, GuildId},
    model::voice::VoiceState,
    prelude::{Context, EventHandler},
};

use super::voice_events;

pub struct Handler;

impl EventHandler for Handler {
    fn voice_state_update(
        &self,
        ctx: Context,
        guild: Option<GuildId>,
        old: Option<VoiceState>,
        new: VoiceState,
    ) {
        let mut ctx = ctx;
        let guild_id = match guild {
            Some(guild_id) => guild_id,
            None => return,
        };

        let new_id: ChannelId = new.channel_id.unwrap_or(ChannelId(0));
        let old_id: ChannelId = match &old {
            Some(old_id) => old_id.channel_id.unwrap_or(ChannelId(0)),
            None => ChannelId(0),
        };

        println!("{} : {}", new_id, old_id);

        if new_id != old_id {
            if old_id != 0 {
                if let Some(channel) = old_id.to_channel(&ctx).unwrap().guild() {
                    voice_events::on_leave(&mut ctx, guild_id, channel, old.unwrap().user_id);
                }
            }
            if new_id != 0 {
                if let Some(channel) = new_id.to_channel(&ctx).unwrap().guild() {
                    voice_events::on_join(&mut ctx, guild_id, channel, new.user_id);
                }
            }
        }
    }
}
