use fcowsay::animalsay;
use rfortune::utils::random_quote;

pub fn say(quotes: &[String]) -> String {
    let output = random_quote(quotes);
    let cow_say = animalsay(output, "cow");
    let formatted_output = format!(
        "```bash \n{}\n```",
        cow_say,
    );
    formatted_output
}