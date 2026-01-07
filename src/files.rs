use rfd::FileDialog;
use std::{fs::File, io::prelude::*, path::PathBuf};

pub fn write_file(file_path: &String, file_text: String) {
    let mut file: File = File::create(file_path).unwrap();
    let _ = file.write_all(file_text.as_bytes());
}

pub fn pick_file() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("rust", &["rs", "toml", "lock"])
        .add_filter("text", &["txt"])
        .add_filter("All files", &["*"])
        .set_title("Choose a file to open in Birdlestein")
        .pick_file()
}
