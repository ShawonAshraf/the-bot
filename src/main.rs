mod emoji_generator;

fn main() {
    let generator = emoji_generator::EmojiGenerator::new();
    let unique_emojis = generator.generate(5);
    
    // convert the emojis to a single string
    let result = unique_emojis.join(" ");
    println!("{:}", result);
}
