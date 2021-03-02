use crate::commandable_ops::Operation;
use anyhow::{bail, Context, Result};
use lazy_static::lazy_static;
use serde_json::{json, Value};
use serenity::model::interactions::Interaction;

pub async fn handle_slash_command(
    ctx: &serenity::client::Context,
    interaction: Interaction,
) -> Result<()> {
    if let Some(data) = interaction.data {
        let text_channel = interaction
            .channel_id
            .to_channel(ctx)
            .await?
            .guild()
            .context("Interaction Guild Channel")?;

        let op = match data.name.as_str() {
            "lock" => Operation::Lock(true),
            "unlock" => Operation::Lock(false),
            "hide" => Operation::Hide(true),
            "unhide" => Operation::Hide(false),
            "help" => Operation::Help,
            _ => bail!("unknown command"),
        };

        let response = op
            .apply(ctx, interaction.member.user.id, &text_channel)
            .await?;

        let out_json = if let Some(response) = response {
            json!({
                "type": 4,
                "data": {
                    "tts": false,
                    "content": response,
                    "embeds": [],
                    "allowed_mentions": []
                }
            })
        } else {
            json!({
                "type": 5
            })
        };

        ctx.http
            .create_interaction_response(interaction.id.0, &interaction.token, &out_json)
            .await
            .context("Slash command response")?;
    }
    Ok(())
}

lazy_static! {
    static ref SLASH_COMMANDS: Vec<Value> = vec![
        json!({
            "name": "help",
            "description": "Some info about Viav",
            "options": [],
        }),
        json!({
            "name": "lock",
            "description": "Lock the voice channel",
            "options": [],
        }),
        json!({
            "name": "unlock",
            "description": "Unlock the voice channel",
            "options": [],
        }),
        json!({
            "name": "hide",
            "description": "Hide the voice channel",
            "options": [],
        }),
        json!({
            "name": "unhide",
            "description": "Unhide the voice channel",
            "options": [],
        }),
    ];
}

pub async fn register_slash_commands(
    ctx: &serenity::client::Context,
) -> Result<(), serenity::Error> {
    for command in SLASH_COMMANDS.iter() {
        ctx.http
            .create_global_application_command(
                ctx.http
                    .get_current_application_info()
                    .await
                    .map(|op| op.id)
                    .unwrap()
                    .0,
                command,
            )
            .await?;
    }
    Ok(())
}
