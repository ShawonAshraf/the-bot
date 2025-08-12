use fcowsay::animalsay;
use rfortune::utils::random_quote;

pub fn say(quotes: &[String], bash_format: bool) -> String {
    let output = random_quote(quotes);
    let cow_say = animalsay(output, "cow");
    
    if bash_format {
        format!("```bash\n{}\n```", cow_say)
    } else {
        cow_say
    }
}