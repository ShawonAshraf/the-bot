use the_bot::guysay::say;

#[test]
fn guysay_plain_contains_quote() {
    let quotes = vec!["Hello, world".to_string()];
    let out = say(&quotes, false);
    assert!(out.contains("Hello, world"));
    // should not be wrapped as bash
    assert!(!out.starts_with("```bash"));
}

#[test]
fn guysay_bash_wrapped() {
    let quotes = vec!["Only one".to_string()];
    let out = say(&quotes, true);
    assert!(out.contains("Only one"));
    assert!(out.starts_with("```bash"));
    assert!(out.ends_with("```"));
}