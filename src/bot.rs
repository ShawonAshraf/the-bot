use std::env;

use crate::emoji_generator::EmojiGenerator;
use crate::guysay::say;
use crate::health_checker::check_health;
use crate::jokes::fetch_joke;
use rand::Rng;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tracing::{debug, error, info};

use std::sync::Arc;
use tokio::sync::RwLock;

pub struct BotState {
    pub quotes: Arc<RwLock<Vec<String>>>,
}

impl BotState {
    pub async fn new(quotes_folder: &str) -> Result<Self, std::io::Error> {
        // Load quotes from the specified folder
        let quotes = crate::quote_loader::load_from_folder(quotes_folder)?;
        Ok(Self {
            quotes: Arc::new(RwLock::new(quotes)),
        })
    }
}

// Define a struct to hold our event handler.
// It doesn't need any data for this simple bot.
struct Handler {
    state: Arc<BotState>,
}

impl Handler {
    pub fn new(state: Arc<BotState>) -> Self {
        Self { state }
    }

    async fn send_simple_reply(
        &self,
        ctx: &Context,
        msg: &Message,
        command_name: &str,
        response: &str,
        error_note: &str,
    ) {
        info!(
            user_id = %msg.author.id,
            username = %msg.author.name,
            channel_id = %msg.channel_id,
            "Processing {} command",
            command_name
        );

        if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
            error!(
                error = ?why,
                channel_id = %msg.channel_id,
                user_id = %msg.author.id,
                "{}",
                error_note
            );
        }
    }
}

// Implement the `EventHandler` trait for our `Handler` struct.
// This trait defines how our bot will react to different events from Discord.
#[async_trait]
impl EventHandler for Handler {
    // This method is called when a new message is created in a channel the bot can see.
    async fn message(&self, ctx: Context, msg: Message) {
        // Check for the `!summon` command.
        // The `starts_with` method checks if the message begins with the specified string.
        // We also check that the `mentions` vector is not empty to ensure a user was tagged.
        if msg.content.starts_with("!summon") && !msg.mentions.is_empty() {
            info!(
                user_id = %msg.author.id,
                username = %msg.author.name,
                channel_id = %msg.channel_id,
                mentions_count = msg.mentions.len(),
                "Processing summon command"
            );

            let emoji_generator: EmojiGenerator = EmojiGenerator::new();
            // Generate a list of unique emojis.
            let unique_emojis: Vec<String> = emoji_generator.generate(7);
            // Convert the emojis to a single string.
            let result: String = unique_emojis.join(" ");

            debug!(emojis = %result, "Generated emojis for summon command");

            if let Err(why) = msg.channel_id.say(&ctx.http, &result).await {
                error!(
                    error = ?why,
                    channel_id = %msg.channel_id,
                    user_id = %msg.author.id,
                    "Failed to send summon command response"
                );
            } else {
                info!(
                    channel_id = %msg.channel_id,
                    user_id = %msg.author.id,
                    emoji_count = unique_emojis.len(),
                    "Successfully sent summon command response"
                );
            }
        }

        // check for the !oracle command
        if msg.content.starts_with("!oracle") {
            info!(
                user_id = %msg.author.id,
                username = %msg.author.name,
                channel_id = %msg.channel_id,
                "Processing oracle command"
            );

            let emoji_generator: EmojiGenerator = EmojiGenerator::new();
            let emoji_count = rand::rng().random_range(5..=15);
            let unique_emojis: Vec<String> = emoji_generator.generate(emoji_count);
            let result: String = unique_emojis.join(" ");

            debug!(
                emojis = %result,
                emoji_count = emoji_count,
                "Generated emojis for oracle command"
            );

            if let Err(why) = msg.channel_id.say(&ctx.http, &result).await {
                error!(
                    error = ?why,
                    channel_id = %msg.channel_id,
                    user_id = %msg.author.id,
                    emoji_count = emoji_count,
                    "Failed to send oracle command response"
                );
            } else {
                info!(
                    channel_id = %msg.channel_id,
                    user_id = %msg.author.id,
                    emoji_count = emoji_count,
                    "Successfully sent oracle command response"
                );
            }
        }

        // Check for the `!joke` command.
        if msg.content.starts_with("!joke") {
            info!(
                user_id = %msg.author.id,
                username = %msg.author.name,
                channel_id = %msg.channel_id,
                "Processing joke command"
            );

            match fetch_joke().await {
                Ok(jokes) => {
                    if let Some(joke) = jokes.first() {
                        let response = format!("🎭 **{}**\n💡 _{}_", joke.setup, joke.punchline);

                        if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
                            error!(
                                error = ?why,
                                channel_id = %msg.channel_id,
                                user_id = %msg.author.id,
                                "Failed to send joke response"
                            );
                        } else {
                            info!(
                                channel_id = %msg.channel_id,
                                user_id = %msg.author.id,
                                "Successfully sent joke response"
                            );
                        }
                    } else {
                        error!(
                            channel_id = %msg.channel_id,
                            user_id = %msg.author.id,
                            "No jokes found in the response"
                        );
                    }
                }
                Err(e) => {
                    error!(
                        error = ?e,
                        channel_id = %msg.channel_id,
                        user_id = %msg.author.id,
                        "Failed to fetch joke"
                    );
                }
            }
        }

        if msg.content.starts_with("!health") {
            info!(
                user_id = %msg.author.id,
                username = %msg.author.name,
                channel_id = %msg.channel_id,
                "Processing health command"
            );

            // avoid ownership movement by cloning msg!
            match check_health(msg.clone().content).await {
                Ok(status) => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, &status).await {
                        error!(error = ?why, "Failed to send health response");
                    }
                }
                Err(e) => {
                    error!(
                        error = ?e,
                        channel_id = %msg.channel_id,
                        user_id = %msg.author.id,
                        "Health check failed"
                    );
                }
            }
        }


        if msg.content.starts_with("!guysay") {
            info!(
                user_id = %msg.author.id,
                username = %msg.author.name,
                channel_id = %msg.channel_id,
                "Processing guysay command"
            );

            let quotes = self.state.quotes.read().await;
            let response = say(&quotes, true);

            if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
                error!(
                    error = ?why,
                    channel_id = %msg.channel_id,
                    user_id = %msg.author.id,
                    "Failed to send guysay response"
                );
            }
        }

        // just single string return blocks
        if msg.content.starts_with("!gaysay") {
            // Disclaimer: this command isn't used to demean people of any community or class
            // :)
            // but this is a typo that people willfully are going to make given the name of the bot
            // besides if you're homophobic, get some help!
            self
                .send_simple_reply(
                    &ctx,
                    &msg,
                    "gaysay",
                    "ব্রো, এসো তোমাকে ব্লেম দেই <3 ",
                    "Cheeky!",
                )
                .await;
        }

        if msg.content.starts_with("!sprint") {
            self
                .send_simple_reply(
                    &ctx,
                    &msg,
                    "sprint",
                    "Sprint in the AI world means, really fast.",
                    "The sprint failed perhaps?",
                )
                .await;
        }

        if msg.content.starts_with("!no") {
            self
                .send_simple_reply(
                    &ctx,
                    &msg,
                    "no",
                    "The no word has deep philosophical meaning to me. It tells me that I can tell anyone, no. Nobody can stop me.",
                    "No!",
                )
                .await;
        }

        if msg.content.starts_with("!breakfast") {
            self
                .send_simple_reply(
                    &ctx,
                    &msg,
                    "breakfast",
                    "I had granola and corn flakes this breakfast, but decided to add AI on top of it anyway.",
                    "breakfast",
                )
                .await;
        }

        if msg.content.starts_with("!PM") {
            self
                .send_simple_reply(
                    &ctx,
                    &msg,
                    "PM",
                    "LONG LIVE THE PM!",
                    "pm",
                )
                .await;
        }

        if msg.content.starts_with("!QA") {
            self
                .send_simple_reply(
                    &ctx,
                    &msg,
                    "!QA",
                    "বাগ পাইসেন? আচ্ছা লিনিয়ারে টিকেট দেন। দেখতেসি বিষয়টা।",
                    "qa",
                )
                .await;
        }

        if msg.content.starts_with("!abubakar") {
            self
                .send_simple_reply(
                    &ctx,
                    &msg,
                    "!abubakar",
                    "All I want for Eid is chunks!",
                    "abubakar"
                )
                .await;
        }

        if msg.content.starts_with("!biriyani") {
            let url = "https://www.youtube.com/watch?v=xvFZjo5PgG0";
            let response = format!("🎭 💡 {}\n", url);

            self
                .send_simple_reply(
                    &ctx,
                    &msg,
                    "!biriyani",
                    response.as_str(),
                    "biriyani"
                )
                .await;
        }

        if msg.content.starts_with("!failed") {
            self.
                send_simple_reply(
                    &ctx,
                    &msg,
                    "!failed",
                    "Don't fix it just revert!",
                    "failed"
                )
                .await;
        }

        if msg.content.starts_with("!talha") {
            self
                .send_simple_reply(
                    &ctx,
                    &msg,
                    "!talha",
                    "আপডেট ছাড়া আরেকবার ডাকলে বেতন 10% মাইনাস",
                    "talha"
                )
                .await;
        }

        if msg.content.starts_with("!jiggu") {
            self
                .send_simple_reply(
                    &ctx,
                    &msg,
                    "!jiggu",
                    "লোকে বলে আমি প্রোজেক্ট ম্যানেজার কিন্তু আমি আসলে আস্ত অপদার্থ, মুনিয়ার মা, প্লেটে আরো থ্যাপলা দাও, খাই।",
                    "jiggu"
                )
                .await;
        }

    }

    // This method is called when the bot is ready to start receiving events.
    async fn ready(&self, _: Context, ready: Ready) {
        // When the bot is ready, we'll log connection details
        info!(
            bot_name = %ready.user.name,
            bot_id = %ready.user.id,
            guild_count = ready.guilds.len(),
            "Discord bot is connected and ready"
        );
    }
}

pub async fn run(quotes_folder: &str) {
    // Get the bot token from the `DISCORD_TOKEN` environment variable.
    info!("Initializing Discord bot");

    let token = match env::var("DISCORD_TOKEN") {
        Ok(token) => {
            info!("Successfully retrieved Discord token from environment");
            token
        }
        Err(e) => {
            error!(error = ?e, "Failed to get DISCORD_TOKEN environment variable");
            panic!("Expected a token in the environment");
        }
    };

    // Define the intents for our bot. Intents tell Discord which events our bot wants to receive.
    // For this bot, we need `GUILD_MESSAGES` to receive message server events,
    // and `MESSAGE_CONTENT` to read the content of the messages.
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    info!("Configured bot intents: GUILD_MESSAGES | MESSAGE_CONTENT");

    // Init bot state
    info!(
        "Initializing bot state with quotes from folder: {}",
        quotes_folder
    );
    let bot_state = Arc::new(match BotState::new(quotes_folder).await {
        Ok(state) => state,
        Err(e) => {
            error!(error = ?e, "Failed to load quotes from folder: {}", quotes_folder);
            panic!("Failed to initialize bot state");
        }
    });

    // Create a new client instance with the token, intents, and our event handler.
    info!("Creating Discord client");
    let mut client = match Client::builder(&token, intents)
        .event_handler(Handler::new(bot_state))
        .await
    {
        Ok(client) => {
            info!("Successfully created Discord client");
            client
        }
        Err(e) => {
            error!(error = ?e, "Failed to create Discord client");
            panic!("Error creating client: {:?}", e);
        }
    };

    // Start the client. This will connect to Discord and start listening for events.
    info!("Starting Discord client connection");
    if let Err(why) = client.start().await {
        error!(
            error = ?why,
            "Discord client encountered a fatal error during startup or runtime"
        );
    } else {
        info!("Discord client shut down gracefully");
    }
}

