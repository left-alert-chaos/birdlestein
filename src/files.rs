use rfd::FileDialog;
use std::{fs::File, io::prelude::*, path::PathBuf};

//wrap file writes and error handling
pub fn write_file(file_path: &String, file_text: String) {
    let mut file: File = File::create(file_path).unwrap();
    let _ = file.write_all(file_text.as_bytes());
}

//use rfd to choose an existing file
pub fn pick_file() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("rust", &["rs", "toml", "lock"])
        .add_filter("text", &["txt"])
        .add_filter("All files", &["*"])
        .set_title("Choose a file to open in Birdlestein")
        .pick_file()
}

//use rfd to choose a file name to save as
pub fn save_file() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("rust", &["rs", "toml", "lock"])
        .add_filter("text", &["txt"])
        .add_filter("All files", &["*"])
        .set_title("Choose a path to save the file.")
        .save_file()
}
