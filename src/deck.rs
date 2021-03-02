use std::convert::TryFrom;

use super::MASTER_USER;
use crate::{commandable_ops::Operation, models::TopicData};
use anyhow::{anyhow, bail, Context};
use serenity::{
    model::prelude::{ChannelId, EmojiId, Message, Reaction, ReactionType, UserId},
    utils::Colour,
};

pub async fn on_deck_reaction(
    ctx: &serenity::client::Context,
    reaction: &Reaction,
    is_add: bool,
) -> anyhow::Result<()> {
    let user = reaction.user(ctx).await?;

    if user.bot {
        return Ok(());
    }

    let text_channel = reaction
        .channel(ctx)
        .await?
        .guild()
        .context("Get text channel of reaction")?;

    let topic = TopicData::try_from(&text_channel)?;

    let is_channel_owner = topic.owner == user.id;
    let is_master_user = MASTER_USER == user.id;
    let is_server_admin = {
        text_channel
            .permissions_for_user(ctx, user.id)
            .await?
            .manage_channels()
    };

    if !is_channel_owner && !is_server_admin && !is_master_user {
        return Ok(());
    }

    Operation::try_from((reaction, is_add))?
        .apply(&ctx, user.id, &text_channel)
        .await?;

    Ok(())
}

pub async fn create_deck(
    ctx: &serenity::client::Context,
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
                .field("Channel Owner", format!("<@{}>", user_id.0), true)
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
            ])
        })
        .await
        .ok()
}

impl TryFrom<(&Reaction, bool)> for Operation {
    type Error = anyhow::Error;

    fn try_from(data: (&Reaction, bool)) -> anyhow::Result<Self> {
        let (reaction, is_add) = data;

        let emoji_name = match &reaction.emoji {
            ReactionType::Custom {
                animated: _,
                id: _,
                name,
            } => name.clone(),
            _ => bail!("Emoji not custom"),
        };

        let emoji_name = emoji_name.context("Emoji name")?;

        match emoji_name.as_str() {
            "lock" => Ok(Operation::Lock(is_add)),
            "eye" => Ok(Operation::Hide(is_add)),
            "alert" => Ok(Operation::NSFW(is_add)),
            _ => Err(anyhow!("")),
        }
    }
}
