use std::env;

use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        id::GuildId,
        interactions::{
            application_command::{
                ApplicationCommand,
                ApplicationCommandInteractionDataOptionValue,
                ApplicationCommandOptionType,
            },
            Interaction,
            InteractionResponseType,
        },
    },
    prelude::*,
};

struct Handler;

pub mod helpers;
use helpers::get_collection::Collection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Collection::get("doodles-official").await?;
    Ok(())
}