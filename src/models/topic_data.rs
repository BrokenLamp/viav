use std::{convert::TryFrom, str::FromStr};

use anyhow::Context;
use serenity::model::{
    channel::GuildChannel,
    id::{ChannelId, UserId},
    Permissions,
};

#[derive(Debug)]
pub struct TopicData {
    pub voice_channel: ChannelId,
    pub owner: UserId,
    pub allow: Permissions,
    pub deny: Permissions,
}

impl TopicData {
    pub async fn voice_channel(
        &self,
        ctx: &serenity::client::Context,
    ) -> anyhow::Result<GuildChannel> {
        let voice_channel = self
            .voice_channel
            .to_channel(ctx)
            .await?
            .guild()
            .context("topic to voice channel")?;
        Ok(voice_channel)
    }
}

impl ToString for TopicData {
    fn to_string(&self) -> String {
        format!(
            "&{}&{}&{}&{}",
            self.voice_channel.0, self.owner.0, self.allow.bits, self.deny.bits
        )
    }
}

impl FromStr for TopicData {
    type Err = anyhow::Error;

    fn from_str(data: &str) -> anyhow::Result<Self> {
        let mut split = data.split('&');
        split.next().context("First next")?;

        let voice_channel = split.next().context("Voice Channel")?;
        let owner = split.next().context("Owner")?;
        let allow = split.next().context("Allow")?;
        let deny = split.next().context("Deny")?;

        Ok(TopicData {
            voice_channel: ChannelId(voice_channel.parse::<u64>()?),
            owner: UserId(owner.parse::<u64>()?),
            allow: Permissions::from_bits_truncate(allow.parse::<u64>()?),
            deny: Permissions::from_bits_truncate(deny.parse::<u64>()?),
        })
    }
}

impl TryFrom<&GuildChannel> for TopicData {
    type Error = anyhow::Error;

    fn try_from(channel: &GuildChannel) -> anyhow::Result<Self> {
        let topic = channel
            .topic
            .as_ref()
            .context("No topic in channel")?
            .clone();
        TopicData::from_str(&topic)
    }
}
