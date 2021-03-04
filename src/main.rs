use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    Args,
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

use std::env;
use lazy_static::lazy_static;

lazy_static! {
    static ref COLOUR_LIST: Vec<&'static str> = {
        vec![
            "Bad Apple!!",
            "Strawberry Crisis!!",
            "Vermillion Halo",
            "Romantic Fall",
            "Morning Glow",
            "Chinese Tea",
            "Soft Doughnut",
            "Salted Caramel",
            "French Croissant",
            "Yellow Temperance",
            "The World",
            "Curse Mind",
            "Darkening Dusk",
            "Native Faith",
            "Dark Road",
            "Last Remote",
            "Hierophant Green",
            "Green-Eyed Jealousy",
            "Ultimate Truth",
            "Lunate Elf",
            "Autumnal Waterfall",
            "Romantic Children",
            "Perfect Freeze",
            "Sky Ruin",
            "Absolute Zero",
            "Desire Drive",
            "Crystallized Silver",
            "Evening Star",
            "Star Platinum",
            "Heartfelt Fancy",
            "INTJ Brain",
            "Sugar Plum",
            "Hot Pink",
            "Lotus Love",
            "Pure Furies",
            "Locked Girl",
            "Titanium White",
            "Little Pebbles",
            "Silver Chariot",
            "Complete Darkness",
        ]
    };
}

#[group]
#[commands(color, colour, leave)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("."))
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

async fn handle_color(ctx: &Context, msg: &Message, args: Args, lang_preference: &str) -> CommandResult {
    let arg_str = args.message();

    let index: Option<usize> = COLOUR_LIST
    .iter()
    .position(|&r| r.to_lowercase() == arg_str.to_lowercase());

    match index {
        None => {
            msg.reply(
                ctx,
                format!(
                    "That is not a valid {} choice. Choices: http://i.imgur.com/PHnCiei.png",
                    lang_preference
                ),
            ).await?;
        }
        Some(index) => match msg.guild_id.unwrap().to_guild_cached(&ctx).await {
            Some(guild) => {
                let color_name = COLOUR_LIST[index];
                match guild.role_by_name(color_name) {
                    Some(role) => {
                        let mut member = msg.member(&ctx).await?;
                        let member_roles = member.roles(&ctx).await;
                        
                        for role in member_roles.unwrap().iter().filter(|r| COLOUR_LIST.contains(&r.name.as_str())) {
                            if let Err(error) = member.remove_role(&ctx, role).await {
                                println!("CANNO REMOVE ROLE:");
                                println!("Details: {:?}", error);
                            }
                        }

                        match member.add_role(&ctx, role).await {
                            Err(error) => {
                                println!("CANNO ADD ROLE:");
                                println!("Details: {:?}", error);
                                msg.reply(
                                    ctx,
                                    format!(
                                        "Error while changing {0} for user {1}, contact the bot owner.",
                                        lang_preference, msg.author.name
                                    ),
                                ).await?;
                            }
                            _ => {
                                msg.reply(
                                    ctx,
                                    format!(
                                        "{0} for **{1}** changed to **{2}**!",
                                        lang_preference, msg.author.name, color_name
                                    ),
                                ).await?;
                            }
                        }
                    }
                    _ => {
                        msg.reply(
                            ctx,
                            format!(
                                "Missing role {} in this guild, contact an admin.",
                                color_name
                            ),
                        ).await?;
                    }
                }
            }
            _ => {
                println!("GUILD not in cache! HOW COULD WE RECEIVE THIS?");
            }
        },
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn color(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    handle_color(ctx, msg, args, "Color").await
}

#[command]
#[only_in(guilds)]
async fn colour(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    handle_color(ctx, msg, args, "Colour").await
}

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    if let Some(guild) = msg.guild_id.unwrap().to_guild_cached(&ctx).await {
        if guild.owner_id == msg.author.id {
            msg.reply(ctx, "Bye!").await?;
            guild.leave(&ctx).await?;
        }
    }

    Ok(())
}
