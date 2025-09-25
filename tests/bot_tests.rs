use std::fs;
use std::io::Write;
use std::path::PathBuf;

use rand::Rng;
use the_bot::bot::BotState;

fn make_temp_dir() -> PathBuf {
    let mut dir = std::env::temp_dir();
    let unique = format!("the_guy_bot_bot_tests_{}", rand::rng().random::<u64>());
    dir.push(unique);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn write_file(path: &PathBuf, name: &str, content: &str) -> PathBuf {
    let mut file_path = path.clone();
    file_path.push(name);
    let mut f = fs::File::create(&file_path).unwrap();
    f.write_all(content.as_bytes()).unwrap();
    file_path
}

#[tokio::test]
async fn bot_state_new_loads_quotes_from_folder() {
    // Arrange: create a temp folder with fortune-formatted files (quotes separated by %)
    let dir = make_temp_dir();
    let _f1 = write_file(&dir, "a.dat", "Hello world\n%\nGeneral Kenobi\n");
    let _f2 = write_file(&dir, "b.dat", "Foo\n%\nBar\n");

    // Act
    let state = BotState::new(dir.to_str().unwrap()).await.expect("BotState::new should succeed for valid folder");

    // Assert
    let quotes = state.quotes.read().await;
    assert_eq!(quotes.len(), 4, "Expected all quotes from both files to be loaded");
    assert!(quotes.contains(&"Hello world".to_string()));
    assert!(quotes.contains(&"General Kenobi".to_string()));
    assert!(quotes.contains(&"Foo".to_string()));
    assert!(quotes.contains(&"Bar".to_string()));
}

#[tokio::test]
async fn bot_state_new_with_missing_folder_returns_error() {
    // Arrange: path that doesn't exist
    let mut dir = std::env::temp_dir();
    dir.push("nonexistent_the_guy_bot_bot_tests_dir");

    // Act
    let res = BotState::new(dir.to_str().unwrap()).await;

    // Assert
    assert!(res.is_err(), "Expected error when folder does not exist");
}
