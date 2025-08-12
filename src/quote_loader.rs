use rfortune::loader;

pub fn load_from_file(file_path: &str) -> Result<Vec<String>, std::io::Error> {
   let path = std::path::Path::new(file_path);
   let quotes = loader::FortuneFile::from_file(path);
    
    if let Ok(quotes) = quotes {
        return Ok(quotes.quotes);
    } else { 
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Failed to load fortunes from file: {:?}", path),
        ));
    }
}

pub fn load_all(file_paths: &[&str]) -> Result<Vec<String>, std::io::Error> {
    let mut all_quotes = Vec::new();
    
    for file_path in file_paths {
        match load_from_file(file_path) {
            Ok(quotes) => all_quotes.extend(quotes),
            Err(e) => return Err(e),
        }
    }
    
    Ok(all_quotes)
}
