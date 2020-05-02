use super::help;
use super::MASTER_USER;
use log::trace;
use serenity::model::prelude::{
    ChannelId, EmojiId, GuildChannel, Message, PermissionOverwrite, PermissionOverwriteType,
    Permissions, Reaction, ReactionType, RoleId, User, UserId,
};
use serenity::prelude::Context;
use serenity::utils::Colour;

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
            let permissions = if is_add {
                PermissionOverwrite {
                    allow: Permissions::empty(),
                    deny: Permissions::READ_MESSAGES,
                    kind: PermissionOverwriteType::Role(RoleId(voice_channel.guild_id.0)),
                }
            } else {
                PermissionOverwrite {
                    allow: Permissions::READ_MESSAGES,
                    deny: Permissions::empty(),
                    kind: PermissionOverwriteType::Role(RoleId(voice_channel.guild_id.0)),
                }
            };
            voice_channel.create_permission(ctx, &permissions).ok();
        }

        "alert" => {
            text_channel.edit(ctx, |e| e.nsfw(is_add)).ok();
        }

        "help" => {
            if is_add {
                help::send_help(&ctx, text_channel.id);

                reaction.delete(&ctx.http).ok();

                let mut my_reaction = reaction.clone();

                trace!("lock   reaction help");
                my_reaction.user_id = ctx.cache.read().user.id;
                trace!("unlock reaction help");
                my_reaction.delete(&ctx.http).ok();
            }
        }

        _ => {}
    }

    Some(())
}

pub fn create_deck(
    ctx: &Context,
    channel_id: ChannelId,
    deck_name: String,
    user_id: UserId,
) -> Option<Message> {
    channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.author(|a| {
                    a.name(deck_name)
                        .icon_url("https://cdn.discordapp.com/attachments/451092625894932493/681741191313883186/Viav.png")
                        .url("https://viav.app/")
                })
                .field("Like Viav?", "[` Vote on Top.gg `](https://top.gg/bot/446151195338473485/vote)", true)
                .field("Owner", format!("<@{}>", user_id.0), true)
                .colour(Colour::from_rgb(103, 58, 183))
            })
            .reactions(vec![
                ReactionType::Custom {
                    animated: false,
                    id: EmojiId(684471911920566281),
                    name: Some(String::from("lock")),
                },
                ReactionType::Custom {
                    animated: false,
                    id: EmojiId(684471928739725376),
                    name: Some(String::from("eye")),
                },
                ReactionType::Custom {
                    animated: false,
                    id: EmojiId(684470685430448128),
                    name: Some(String::from("alert")),
                },
                ReactionType::Custom {
                    animated: false,
                    id: EmojiId(684471126130425935),
                    name: Some(String::from("help")),
                },
            ])
        })
        .ok()
}

pub fn get_deck_reaction_info(
    ctx: &Context,
    reaction: &Reaction,
) -> Option<(GuildChannel, GuildChannel, User)> {
    if reaction.user(ctx).ok()?.bot {
        return None;
    }

    let text_channel = {
        trace!("lock   get deck reaction info 1");
        reaction.channel(ctx).ok()?.guild()?.read().clone()
    };
    trace!("unlock get deck reaction info 1");

    let topic = text_channel.topic.as_ref()?.clone();
    let mut topic = topic.split("&");

    topic.next()?;

    let voice_channel = {
        trace!("lock   get deck reaction info 2");
        ChannelId(topic.next()?.parse::<u64>().ok()?)
            .to_channel(ctx)
            .ok()?
            .guild()?
            .read()
            .clone()
    };
    trace!("unlock get deck reaction info 2");

    let owner = UserId(topic.next()?.parse::<u64>().ok()?)
        .to_user(ctx)
        .ok()?;

    let is_channel_owner = owner.id == reaction.user_id;
    let is_master_user = MASTER_USER == reaction.user_id;
    let is_server_admin = {
        let perms = {
            trace!("lock   get deck reaction info 3");
            reaction
                .channel(ctx)
                .ok()?
                .guild()?
                .read()
                .permissions_for_user(ctx, reaction.user_id)
        };
        trace!("unlock get deck reaction info 3");
        perms.ok()?.manage_channels()
    };

    if is_channel_owner || is_server_admin || is_master_user {
        Some((voice_channel, text_channel, owner))
    } else {
        None
    }
}
