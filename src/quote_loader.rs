use rfortune::loader;

pub fn load_from_file(file_path: &str) -> Result<Vec<String>, std::io::Error> {
   let path = std::path::Path::new(file_path);
   let quotes = loader::FortuneFile::from_file(path);

    if let Ok(quotes) = quotes {
        Ok(quotes.quotes)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Failed to load fortunes from file: {:?}", path),
        ))
    }
}


pub fn load_from_folder(folder_path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut all_quotes = Vec::new();
    let path = std::path::Path::new(folder_path);
    
    if path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            if entry.path().is_file() {
                match load_from_file(entry.path().to_str().unwrap()) {
                    Ok(quotes) => all_quotes.extend(quotes),
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(all_quotes)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Folder not found: {:?}", path),
        ))
    }
}
