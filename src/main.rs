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

        // Skip the root directory "."
        if entry.file_name() == "." {
            continue;
        }

        // Only show items in the current directory (depth 1)
        if entry.depth() > 1 {
            continue;
        }

        if entry.path().is_dir() {
            println!(
                "{:>10} {}",
                "-",
                entry.file_name().display().to_string().blue()
            );
        } else if entry.path().is_file() {
            match find_length(entry.path().to_str().unwrap()) {
                Ok(len) => println!("{:>10} {}", len, entry.file_name().display()),
                Err(e) => eprintln!("Error reading {}: {}", entry.path().display(), e),
            }
        }
    }
}
