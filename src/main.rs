use colored::*;
use std::fs;
use std::io;
use walkdir::WalkDir;
fn find_length(path: &str) -> io::Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}
fn main() {
    println!("{}", "Length Name".green());
    for entry in WalkDir::new(".") {
        let entry = entry.unwrap();

        // Skip directories, only process files
        if entry.path().is_file() {
            match find_length(entry.path().to_str().unwrap()) {
                Ok(len) => println!("{} {}", len, entry.file_name().display()),
                Err(e) => eprintln!("Error reading {}: {}", entry.path().display(), e),
            }
        }
    }
}
