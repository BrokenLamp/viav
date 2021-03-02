use anyhow::Context;
use async_trait::async_trait;
use serenity::model::id::ChannelId;

#[async_trait]
pub trait HandleError {
    async fn handle(&self, ctx: &serenity::client::Context, text_channel: Option<ChannelId>);
}

#[async_trait]
impl<T: Send + Sync> HandleError for anyhow::Result<T> {
    async fn handle(&self, ctx: &serenity::client::Context, text_channel: Option<ChannelId>) {
        if let Err(err) = self {
            if let Some(text_channel) = text_channel {
                let result = text_channel
                    .send_message(ctx, |m| m.content(format!("{:?}", err)))
                    .await
                    .context("Send error message to channel");
                if let Err(err) = result {
                    eprintln!("{:?}", err);
                }
            } else {
                eprintln!("{:?}", err);
            }
        }
    }
}
