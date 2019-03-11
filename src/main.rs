#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

extern crate dotenv;
extern crate env_logger;
extern crate serenity;

use serenity::model::channel::Message;
use serenity::model::id::*;
use serenity::prelude::*;
use std::env;
use std::str::SplitWhitespace;

struct Handler;

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

impl Handler {
    fn send_message_safe(&self, message: String, channel_id: ChannelId) {
        if let Err(error) = channel_id.say(message) {
            warn!("Error while sending message: {:?}", error);
        }
    }

    fn cmd_colour(
        &self,
        _ctx: Context,
        message: Message,
        colour_iter: SplitWhitespace,
        lang_preference: &str,
    ) {
        let args: Vec<&str> = colour_iter.collect();
        let arg_str = args.as_slice().join(" ");
        let index: Option<usize> = COLOUR_LIST
            .iter()
            .position(|&r| r.to_lowercase() == arg_str.to_lowercase());

        match index {
            None => {
                self.send_message_safe(
                    format!(
                        "That is not a valid {} choice. Choices: http://i.imgur.com/PHnCiei.png",
                        lang_preference
                    ),
                    message.channel_id,
                );
            }
            Some(index) => match message.guild_id.unwrap().to_guild_cached() {
                Some(arc) => {
                    let color_name = COLOUR_LIST[index];
                    match arc.read().role_by_name(color_name) {
                        Some(role) => {
                            let mut member = message.member().unwrap();
                            let member_roles = member.roles().unwrap();
                            member_roles
                                .iter()
                                .filter(|r| COLOUR_LIST.contains(&r.name.as_str()))
                                .for_each(|r| {
                                    if let Err(error) = member.remove_role(r) {
                                        warn!("CANNO REMOVE ROLE:");
                                        warn!("Details: {:?}", error);
                                    }
                                });

                            match member.add_role(role) {
                                Err(error) => {
                                    warn!("CANNO ADD ROLE:");
                                    warn!("Details: {:?}", error);
                                    self.send_message_safe(
                                        format!(
                                            "Error while changing {0} for user {1}, contact the bot owner.",
                                            lang_preference, message.author.name
                                        ),
                                        message.channel_id,
                                    );
                                }
                                _ => {
                                    self.send_message_safe(
                                        format!(
                                            "{0} for **{1}** changed to **{2}**!",
                                            lang_preference, message.author.name, color_name
                                        ),
                                        message.channel_id,
                                    );
                                }
                            }
                        }
                        _ => {
                            self.send_message_safe(
                                format!(
                                    "Missing role {} in this guild, contact an admin.",
                                    color_name
                                ),
                                message.channel_id,
                            );
                        }
                    }
                }
                _ => {
                    warn!("GUILD not in cache! HOW COULD WE RECEIVE THIS?");
                }
            },
        }
    }

    fn cmd_help(&self, _ctx: Context, message: Message) {
        message
            .channel_id
            .send_message(|m| {
                m.embed(|e| {
                    e.author(|a| {
                        a.name("YoumuBot Help")
                            .icon_url("http://i.imgur.com/6rDYlAI.png")
                    }).title("Commands:")
                        .description("Note: All commands are case sensitive.")
                        .colour(0x00CB_8B83)
                        .field(".help", "Gives you the list of commands, as shown here.", false)
                        .field(".colour <colour>", "Changes colour for the user. No argument or invalid argument will remove your current color and give you the list of colors to choose from.", false)
                        .field(".color <color>", "Does the same thing as .colour, but for y'all Americans.", false)
                })
            })
            .ok();
    }

    fn cmd_leave(&self, _ctx: Context, message: Message) {
        if let Some(arc) = message.guild_id.unwrap().to_guild_cached() {
            let guild = arc.read();
            if guild.owner_id == message.author.id {
                message.channel_id.say("Bye!").ok();
                guild.leave().ok();
            }
        }
    }
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, message: Message) {
        let msg = message.content_safe();
        let mut iterator: SplitWhitespace = msg.split_whitespace();
        match (iterator.next(), message.guild_id) {
            (Some(".color"), Some(_)) => self.cmd_colour(ctx, message, iterator, "Color"),
            (Some(".colour"), Some(_)) => self.cmd_colour(ctx, message, iterator, "Colour"),
            (Some(".help"), _) => self.cmd_help(ctx, message),
            (Some(".leave"), Some(_)) => self.cmd_leave(ctx, message),
            _ => {}
        };
    }
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
