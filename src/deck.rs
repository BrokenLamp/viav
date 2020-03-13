use serenity::model::prelude::{ChannelId, GuildChannel, Reaction, ReactionType, User, UserId};
use serenity::prelude::Context;

pub fn on_deck_reaction(
    ctx: &Context,
    reaction: &Reaction,
    is_add: bool,
    voice_channel: &mut GuildChannel,
    text_channel: &mut GuildChannel,
    _owner: User,
) -> Option<()> {
    let emoji_name = match &reaction.emoji {
        ReactionType::Custom {
            animated: _,
            id: _,
            name,
        } => name.clone()?,
        _ => return None,
    };

    match emoji_name.as_str() {
        "lock" => {
            voice_channel
                .edit(ctx, |e| e.user_limit(is_add as u64))
                .ok();
        }

        "eye" => {
            println!("eye");
        }

        "alert" => {
            text_channel.edit(ctx, |e| e.nsfw(is_add)).ok();
        }

        "help" => {
            println!("help");
        }

        _ => {}
    }

    Some(())
}

pub fn get_deck_reaction_info(
    ctx: &Context,
    reaction: &Reaction,
) -> Option<(GuildChannel, GuildChannel, User)> {
    if reaction.user(ctx).ok()?.bot {
        return None;
    }

    let text_channel = reaction.channel(ctx).ok()?.guild()?.read().clone();

    let topic = text_channel.topic.as_ref()?.clone();
    let mut topic = topic.split("&");

    topic.next()?;

    let voice_channel = ChannelId(topic.next()?.parse::<u64>().ok()?)
        .to_channel(ctx)
        .ok()?
        .guild()?
        .read()
        .clone();

    let owner = UserId(topic.next()?.parse::<u64>().ok()?)
        .to_user(ctx)
        .ok()?;

    let is_channel_owner = owner.id.0 == reaction.user_id.0;
    let is_server_admin = {
        reaction
            .channel(ctx)
            .ok()?
            .guild()?
            .read()
            .permissions_for_user(ctx, reaction.user_id)
            .ok()?
            .manage_channels()
    };

    if !is_channel_owner && !is_server_admin {
        return None;
    }

    Some((voice_channel, text_channel, owner))
}
