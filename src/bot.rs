use std::env;

use crate::emoji_generator::EmojiGenerator;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

// Define a struct to hold our event handler.
// It doesn't need any data for this simple bot.
struct Handler;

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
            let emoji_generator = EmojiGenerator::new();
            // Generate a list of unique emojis.
            let unique_emojis = emoji_generator.generate(7);
            // Convert the emojis to a single string.
            let result = unique_emojis.join(" ");

            if let Err(why) = msg.channel_id.say(&ctx.http, result).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // This method is called when the bot is ready to start receiving events.
    async fn ready(&self, _: Context, ready: Ready) {
        // When the bot is ready, we'll print its username to the console.
        println!("{} is connected!", ready.user.name);
    }
}

pub async fn run() {
    // Get the bot token from the `DISCORD_TOKEN` environment variable.
    // The `.expect()` method will cause the program to panic if the variable is not set.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Define the intents for our bot. Intents tell Discord which events our bot wants to receive.
    // For this bot, we need `GUILD_MESSAGES` to receive message events in servers,
    // and `MESSAGE_CONTENT` to read the content of the messages.
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    // Create a new client instance with the token, intents, and our event handler.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start the client. This will connect to Discord and start listening for events.
    // The `if let Err` block will print an error message if the client fails to start.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
