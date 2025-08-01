mod emoji_generator;
mod clipboard;

fn main() {
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
