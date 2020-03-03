use serenity::model::prelude::{ChannelId, GuildChannel, Reaction, ReactionType, User, UserId};
use serenity::prelude::{Context, RwLock};
use std::sync::Arc;

pub fn on_deck_reaction_add(
    ctx: &Context,
    reaction: &Reaction,
    voice_channel: &mut GuildChannel,
    text_channel: &mut GuildChannel,
    owner: User,
) -> Option<()> {
    let emoji_id = match &reaction.emoji {
        ReactionType::Custom {
            animated: _,
            id,
            name: _,
        } => id.0,
        _ => return None,
    };

    match emoji_id {
        // Lock
        684471911920566281 => {
            voice_channel.edit(ctx, |e| e.user_limit(1)).ok()?;
        }

        // Eye
        684471928739725376 => println!("eye"),

        // Alert
        684470685430448128 => {
            text_channel.edit(ctx, |e| e.nsfw(true));
        }

        // Help
        684471126130425935 => println!("help"),

        _ => {}
    }

    Some(())
}

pub fn on_deck_reaction_remove(
    ctx: &Context,
    reaction: &Reaction,
    voice_channel: &mut GuildChannel,
    text_channel: &mut GuildChannel,
    owner: User,
) -> Option<()> {
    let emoji_id = match &reaction.emoji {
        ReactionType::Custom {
            animated: _,
            id,
            name: _,
        } => id.0,
        _ => return None,
    };

    match emoji_id {
        // Lock
        684471911920566281 => {
            voice_channel.edit(ctx, |e| e.user_limit(0)).ok()?;
        }

        // Eye
        684471928739725376 => println!("eye"),

        // Alert
        684470685430448128 => {
            text_channel.edit(ctx, |e| e.nsfw(false));
        }

        // Help
        684471126130425935 => println!("help"),

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

    if owner.id.0 != reaction.user_id.0 {
        return None;
    }

    Some((voice_channel, text_channel, owner))
}
