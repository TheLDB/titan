extern crate dotenv;

use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        id::GuildId,
        interactions::{
            application_command::{
                // ApplicationCommand,
                ApplicationCommandInteractionDataOptionValue,
                ApplicationCommandOptionType,
            },
            Interaction,
            InteractionResponseType,
        },
    },
    prelude::*,
};

use std::env;
use dotenv::dotenv;
pub struct Handler;
use crate::helpers;

pub struct Returnable {
    collection: Option<helpers::get_collection::Collection>,
}
#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "collection" => {
                    let options = command
                    .data
                    .options
                    .get(0)
                    .expect("Expected collection slug")
                    .resolved
                    .as_ref()
                    .expect("Expected collection slug?");

                if let ApplicationCommandInteractionDataOptionValue::String(slug) = options {
                    let lower = slug.to_lowercase();
                    let result = helpers::get_collection::Collection::get(&lower.to_string()).await;
                    // let unwr_res = result.unwrap();
                    if let Ok(unwr_res) = result {
                        if unwr_res.status == 200 {
                            Returnable {
                                collection: Some(unwr_res)
                            }
                        }
                        else {
                            Returnable {
                                collection: None,
                            }
                        }
                    }
                    else {
                        Returnable {
                            collection: None,
                        }
                    }
                    
                }
                else {
                    Returnable {
                        collection: None,
                    }
                }
                },
                // if no match
                _ => Returnable {
                    // exists: Some(false),
                    collection: None,
                },
            };

            match command.data.name.as_str() {
                "collection" => {
                    if let Err(why) = command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| {
                                    if let Some(or) = content.collection {
                                        let name = or.name;
                                        let formatted = format!("{}'s Floor Price: {} Ξ", name.replace('"', ""), or.fp);
                                        message.content(formatted)
                                    }
                                    else {
                                        message.content("Couldn't find collection!")
                                    }
                                })
                        }).await {
                            println!("Cannot respond to slash command: {}", why);
                        }
                },
                "global" => {
                    if let Err(why) = command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                println!("we hit!");
                                message.content("we hit!")
                            })
                    }).await {
                        println!("Cannot respond to slash command: {}", why);
                    }
                }
                &_ => ()
            };
        }
        else {
            println!("wtf")
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        dotenv().ok();
        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let _commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
            // Set commands with info abt them?
                .create_application_command(|command| {
                    command.name("collection").description("Get information about an OpenSea Collection").create_option(|option| {
                        option
                            .name("slug")
                            .description("The collection slug to fetch")
                            .kind(ApplicationCommandOptionType::String)
                            .required(true)
                    })
                })

        })
        .await;

        // println!("I now have the following guild slash commands: {:#?}", _commands);

        // let _guild_command =
        //     ApplicationCommand::create_global_application_command(&ctx.http, |command| {
        //         command.name("collection-global").description("Get information about an OpenSea Collection").create_option(|option| {
        //             option
        //                 .name("slug")
        //                 .description("The collection slug to fetch")
        //                 .kind(ApplicationCommandOptionType::String)
        //                 .required(true)
        //         })
        //     })
        //     .await;

        // println!("I created the following global slash command: {:#?}", guild_command);
    }
}