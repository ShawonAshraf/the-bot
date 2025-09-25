use std::fs;
use std::io::Write;
use std::path::PathBuf;

use rand::Rng;
use the_bot::quote_loader::{load_from_file, load_from_folder};

fn make_temp_dir() -> PathBuf {
    let mut dir = std::env::temp_dir();
    let unique = format!("the_guy_bot_test_{}", rand::rng().random::<u64>());
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

#[test]
fn load_from_file_parses_fortunes() {
    let dir = make_temp_dir();
    let file = write_file(&dir, "quotes.dat", "First quote\n%\nSecond quote\n");

    let quotes = load_from_file(file.to_str().unwrap()).unwrap();
    assert_eq!(quotes.len(), 2);
    assert!(quotes.contains(&"First quote".to_string()));
    assert!(quotes.contains(&"Second quote".to_string()));
}

#[test]
fn load_from_folder_aggregates_files() {
    let dir = make_temp_dir();
    let _f1 = write_file(&dir, "a.dat", "A1\n%\nA2\n");
    let _f2 = write_file(&dir, "b.dat", "B1\n%\nB2\n");

    let quotes = load_from_folder(dir.to_str().unwrap()).unwrap();
    assert_eq!(quotes.len(), 4);
}

#[test]
fn load_from_folder_missing_returns_error() {
    let mut dir = std::env::temp_dir();
    dir.push("nonexistent_the_guy_bot_dir");
    let res = load_from_folder(dir.to_str().unwrap());
    assert!(res.is_err());
}
