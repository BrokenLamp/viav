use serenity::{model::id::GuildId, model::voice::VoiceState, prelude::Context};

pub fn on_join(_ctx: &mut Context, _guild_id: GuildId, _voice_state: VoiceState) -> Option<()> {
    println!("JOIN");
    Some(())
}

pub fn on_leave(
    _ctx: &mut Context,
    _guild_id: GuildId,
    _voice_state: Option<VoiceState>,
) -> Option<()> {
    println!("LEAVE");
    Some(())
}
