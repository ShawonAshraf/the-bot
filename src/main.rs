mod bot;
mod clipboard;
mod emoji_generator;

use std::env;

#[tokio::main]
async fn main() {
    let args = env::args().collect::<Vec<String>>();
    
    // Check if the user provided a command-line argument
    if args.len() > 1 && args[1] == "bot" {
        // If the argument is "bot", run the bot
        println!("Running the Discord bot...");
        bot::run().await;
        return;
    }
    else {
        // If no argument or a different argument is provided, run the emoji generator
        let generator = emoji_generator::EmojiGenerator::new();
        let unique_emojis = generator.generate(5);

        // convert the emojis to a single string
        let result = unique_emojis.join(" ");

        // copy to clipboard
        println!("Copying to clipboard: {}", result);

        match clipboard::copy_to_clipboard(&result) {
            Ok(_) => println!("Emojis copied to clipboard successfully!"),
            Err(e) => eprintln!("Error copying to clipboard: {}", e),
        }
    }
}
