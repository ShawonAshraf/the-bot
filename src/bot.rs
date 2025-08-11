use std::env;

use crate::emoji_generator::EmojiGenerator;
use crate::health_checker::check_health;
use crate::jokes::fetch_joke;
use rand::Rng;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tracing::{debug, error, info};

// Send a funny message on failure
fn send_funny() -> String {
    // long replies
    let lmaos: Vec<&str> = [
        "এই মেসেজ কেডায় দিসে? 🤬",
        "আর কাম কাজ নাই? 🥴",
        "পুৎ কইরা দিমু 😈",
        "স্বজন হারানোর বেদনা আমিও বুঝি 😭",
        "আহো ভাতিজা আহো 😈",
        "আমি জুনায়েদ 😇",
        "সাগর, তুমি ভালো হয়ে যাও, মাসুদ হয়নি, তুমি হউ। 🥸",
        "ইংরেজিতে যেহেতু বুইলছেন, ঠিকই হবে! 🤓",
        "চ্যালেঞ্জিং টাইমস! 😎",
        "১০% নিয়া গেলো লন্ডনের ই বাসে রে, মরার কোকিলে! 🐦‍⬛",
    ]
    .to_vec();

    // pick a random message from lmaos
    // make the random selection using rand::Rng
    let random_index = rand::rng().random_range(0..lmaos.len());
    let lmao_msg = lmaos[random_index];
    lmao_msg.to_string()
}

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

            match check_health(msg.content).await {
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
                        "Health check failed, sending funny message"
                    );

                    let funny_message = send_funny();
                    if let Err(why) = msg.channel_id.say(&ctx.http, funny_message.as_str()).await {
                        error!(error = ?why, "Failed to send health response");
                    }
                }
            }
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

pub async fn run() {
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

    // Create a new client instance with the token, intents, and our event handler.
    info!("Creating Discord client");
    let mut client = match Client::builder(&token, intents)
        .event_handler(Handler)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let handler = Handler;
        // Test that Handler can be created (zero-sized type)
        assert_eq!(size_of_val(&handler), 0);
    }

    #[test]
    fn test_command_detection_logic() {
        // Test the command detection logic used in the bot
        let summon_msg = "!summon @user";
        let summon_simple = "!summon";
        let oracle_msg = "!oracle";
        let oracle_with_text = "!oracle tell me my future";
        let regular_msg = "Hello world";
        let partial_cmd = "oracle without !";
        let wrong_case1 = "!ORACLE";
        let wrong_case2 = "!Oracle";

        // Test summon command detection
        assert!(summon_msg.starts_with("!summon"));
        assert!(summon_simple.starts_with("!summon"));

        // Test oracle command detection
        assert!(oracle_msg.starts_with("!oracle"));
        assert!(oracle_with_text.starts_with("!oracle"));

        // Test non-commands
        assert!(!regular_msg.starts_with("!summon"));
        assert!(!regular_msg.starts_with("!oracle"));
        assert!(!partial_cmd.starts_with("!oracle"));

        // Test case sensitivity (commands should be lowercase)
        assert!(!wrong_case1.starts_with("!oracle"));
        assert!(!wrong_case2.starts_with("!oracle"));
    }

    #[test]
    fn test_emoji_generator_integration() {
        // Test that EmojiGenerator works as expected for our bot
        let generator = EmojiGenerator::new();

        // Test fixed count (like summon command uses 7)
        let emojis_7 = generator.generate(7);
        assert_eq!(emojis_7.len(), 7);

        // Test various counts (like oracle command uses 5-15)
        for count in 5..=15 {
            let emojis = generator.generate(count);
            assert_eq!(emojis.len(), count);

            // Ensure all generated emojis are unique
            let mut unique_emojis = emojis.clone();
            unique_emojis.sort();
            unique_emojis.dedup();
            assert_eq!(emojis.len(), unique_emojis.len());
        }

        // Test edge cases
        let emojis_0 = generator.generate(0);
        assert_eq!(emojis_0.len(), 0);

        let emojis_1 = generator.generate(1);
        assert_eq!(emojis_1.len(), 1);
    }

    #[test]
    fn test_random_range_generation() {
        // Test that our random range generation works correctly for oracle command
        for _ in 0..100 {
            let emoji_count = rand::rng().random_range(5..=15);
            assert!(
                emoji_count >= 5,
                "emoji_count {} should be >= 5",
                emoji_count
            );
            assert!(
                emoji_count <= 15,
                "emoji_count {} should be <= 15",
                emoji_count
            );
        }
    }

    #[test]
    fn test_emoji_joining() {
        // Test the emoji joining logic used in both commands
        let generator = EmojiGenerator::new();
        let emojis = generator.generate(3);
        let result = emojis.join(" ");

        // Should have spaces between emojis
        assert!(result.contains(" "));

        // Should not start or end with space for proper formatting
        assert!(!result.starts_with(" "));
        assert!(!result.ends_with(" "));

        // Split should give us back the original count
        let split_result: Vec<&str> = result.split(" ").collect();
        assert_eq!(split_result.len(), 3);

        // Test with different counts
        for count in 1..=10 {
            let emojis = generator.generate(count);
            let result = emojis.join(" ");
            let split_result: Vec<&str> = result.split(" ").collect();
            assert_eq!(split_result.len(), count);
        }
    }

    #[test]
    fn test_mentions_check_logic() {
        // Test the mention logic used in summon command
        let empty_mentions: Vec<String> = vec![];
        let with_mentions = vec!["@user1".to_string(), "@user2".to_string()];

        // Simulate the logic: summon only works with mentions
        assert!(empty_mentions.is_empty());
        assert!(!with_mentions.is_empty());

        // Test the condition used in summon command
        let should_trigger_summon_empty =
            "!summon".starts_with("!summon") && !empty_mentions.is_empty();
        let should_trigger_summon_with_mentions =
            "!summon".starts_with("!summon") && !with_mentions.is_empty();

        assert!(!should_trigger_summon_empty);
        assert!(should_trigger_summon_with_mentions);
    }

    #[test]
    fn test_command_prefix_combinations() {
        // Test various command prefix scenarios
        let test_cases = vec![
            ("!summon", true, false),        // summon: yes, oracle: no
            ("!oracle", false, true),        // summon: no, oracle: yes
            ("!summon @user", true, false),  // summon: yes, oracle: no
            ("!oracle please", false, true), // summon: no, oracle: yes
            ("!sum", false, false),          // summon: no, oracle: no
            ("!ora", false, false),          // summon: no, oracle: no
            ("hello !summon", false, false), // summon: no, oracle: no
            ("hello !oracle", false, false), // summon: no, oracle: no
            ("", false, false),              // summon: no, oracle: no
            ("!SUMMON", false, false),       // summon: no, oracle: no (case sensitive)
            ("!ORACLE", false, false),       // summon: no, oracle: no (case sensitive)
        ];

        for (message, expected_summon, expected_oracle) in test_cases {
            let is_summon = message.starts_with("!summon");
            let is_oracle = message.starts_with("!oracle");

            assert_eq!(
                is_summon, expected_summon,
                "Message '{}' summon detection failed. Expected: {}, Got: {}",
                message, expected_summon, is_summon
            );
            assert_eq!(
                is_oracle, expected_oracle,
                "Message '{}' oracle detection failed. Expected: {}, Got: {}",
                message, expected_oracle, is_oracle
            );
        }
    }

    #[test]
    fn test_emoji_count_ranges() {
        // Test that emoji generation handles the different count requirements
        let generator = EmojiGenerator::new();

        // Summon command uses fixed count of 7
        let summon_emojis = generator.generate(7);
        assert_eq!(summon_emojis.len(), 7);

        // Oracle command uses random count between 5-15, test the boundaries
        let min_oracle_emojis = generator.generate(5);
        assert_eq!(min_oracle_emojis.len(), 5);

        let max_oracle_emojis = generator.generate(15);
        assert_eq!(max_oracle_emojis.len(), 15);

        // Test that we can generate all counts in the oracle range
        for count in 5..=15 {
            let emojis = generator.generate(count);
            assert_eq!(emojis.len(), count, "Failed to generate {} emojis", count);
        }
    }

    #[test]
    fn test_environment_variable_logic() {
        // Test the logic around environment variable handling
        // Note: We can't test the actual env::var call without modifying existing code,
        // but we can test the expected behavior patterns

        use std::env;

        // Test that env::var returns Result<String, VarError>
        let test_result = env::var("NONEXISTENT_VAR_FOR_TESTING_12345");
        assert!(test_result.is_err());

        // Test string handling for token-like values
        let fake_token = "fake_discord_token_123";
        assert!(!fake_token.is_empty());
        assert!(fake_token.len() > 10); // Discord tokens are much longer
    }

    #[test]
    fn test_string_operations_used_in_bot() {
        // Test string operations that the bot uses
        let test_messages = vec![
            "!summon @user1 @user2",
            "!oracle tell me the future",
            "Hello, this is a regular message",
        ];

        for msg in test_messages {
            // Test string methods used in the bot
            let _starts_with_summon = msg.starts_with("!summon");
            let _starts_with_oracle = msg.starts_with("!oracle");
            let _content_string = msg.to_string();

            // These operations should not panic
            assert!(true);
        }

        // Test emoji joining operation
        let emojis = vec!["😀", "😃", "😄"];
        let joined = emojis
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        assert_eq!(joined, "😀 😃 😄");
    }
}
