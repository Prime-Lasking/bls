use walkdir::WalkDir;
fn main() {
    for entry in WalkDir::new(".") {
        // iterate over all entries in the given directory
        let entry = entry.unwrap(); // unwrap the result, assuming it's valid
        if entry.file_name() != "." {
            println!("{}", entry.file_name().display()); // Print the path of the matching entry
        }
    }
}
