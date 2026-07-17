use chrono::{DateTime, Local};
use clap::Parser;
use colored::Colorize;
use std::fs;
use std::io;
use std::path::Path;

fn get_last_modified(path: &Path) -> String {
    fs::metadata(path)
        .and_then(|m| m.modified())
        .map(|t| {
            let dt: DateTime<Local> = t.into();
            format!("{}", dt.format("%Y-%m-%d %H:%M"))
        })
        .unwrap_or_else(|_| "???".to_string())
}

#[derive(Parser)]
#[command(name = "bls", about = "A better ls")]
struct Args {
    /// Directory to list
    path: Option<String>,

    /// Show hidden files
    #[arg(short, long)]
    all: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let path = args.path.as_deref().unwrap_or(".");
    println!(
        "{:>20} {:>10}   {}",
        "LastWriteTime".green(),
        "Length".green(),
        "Name".green()
    );

    // first: directories
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            if entry.file_name().to_string_lossy().starts_with('.') && !args.all {
                continue;
            } else {
                println!(
                    "{:>20} {:>10}   {}",
                    get_last_modified(&entry.path()).blue(),
                    "-".blue(),
                    entry.file_name().to_string_lossy().blue()
                );
            }
        }
    }

    // then: files
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_file() {
            if entry.file_name().to_string_lossy().starts_with('.') && !args.all {
                continue;
            } else {
                println!(
                    "{:>20} {:>10}   {}",
                    get_last_modified(&entry.path()),
                    metadata.len(),
                    entry.file_name().to_string_lossy()
                );
            }
        }
    }
    Ok(())
}
