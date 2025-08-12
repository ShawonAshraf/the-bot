mod bot;
mod clipboard;
mod emoji_generator;
mod guysay;
mod health_checker;
mod jokes;
mod quote_loader;

use std::env;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber with structured logging
    tracing_subscriber::fmt::init();

    info!("Starting Summoner Emoji Bot application");

    let args = env::args().collect::<Vec<String>>();

    // Check if the user provided a command-line argument
    if args.len() > 2 && args[1] == "bot" {
        // get folder dir
        let folder_path = &args[2];
        // assure that the folder path exists
        if std::path::Path::new(folder_path).exists() {
            info!("Starting bot with folder: {}", folder_path);
            // Start the bot with the provided folder path
            bot::run(folder_path).await;
            return;
        } else {
            error!("Folder does not exist: {}", folder_path);
        }
    } else if args.len() > 1 && args[1] == "emoji" {
        // If no argument or a different argument is provided, run the emoji generator
        info!("Starting emoji generator mode");
        let generator = emoji_generator::EmojiGenerator::new();
        let unique_emojis = generator.generate(5);

        // convert the emojis to a single string
        let result = unique_emojis.join(" ");

        // copy to clipboard
        info!(emojis = %result, "Generated emojis, copying to clipboard");

        match clipboard::copy_to_clipboard(&result) {
            Ok(_) => info!("Emojis copied to clipboard successfully"),
            Err(e) => error!(error = %e, "Failed to copy emojis to clipboard"),
        }
    } else if args.len() > 2 && args[1] == "guysay" {
        // get files dir from args[2]
        info!("Starting guysay mode");
        let folder_path = &args[2];

        // check if the folder exists and if it does load quotes
        if std::path::Path::new(folder_path).exists() {
            match quote_loader::load_from_folder(folder_path) {
                Ok(quotes) => {
                    info!(
                        "Loaded {} quotes from folder: {}",
                        quotes.len(),
                        folder_path
                    );
                    info!("Finding a random quote to say");
                    let quote = guysay::say(&quotes, false);
                    println!("{}", quote);
                }
                Err(e) => error!(error = %e, "Failed to load quotes from folder"),
            }
        } else {
            error!("Folder does not exist: {}", folder_path);
        }
    } else {
        error!("Invalid arguments supplied");
    }
}
