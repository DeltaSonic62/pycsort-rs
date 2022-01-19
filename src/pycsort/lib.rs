use chrono::{offset, DateTime};
use glob::{glob_with, MatchOptions};
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_files(folder: &str) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();

    let file_types = [
        ".jpg", ".jpeg", ".png", ".heic", ".webp", ".tiff", ".tif", ".gif", ".mp4", ".mpeg4",
        ".hevc", ".webm", ".mkv", ".avi", ".wmv", ".m4v",
    ];

    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    for file_type in file_types {
        let pattern = String::from(folder) + "/**/*" + &file_type.to_string();
        for entry in glob_with(&pattern, options).expect("Could not read from selected folder") {
            match entry {
                Ok(path) => files.push(path),
                Err(e) => eprintln!("{:?}", e),
            }
        }
    }

    files
}

pub fn get_file_metadata(path: &Path) -> String {
    let md = fs::metadata(path).expect("No metadata found");
    if let Ok(time) = md.created() {
        let dt: DateTime<offset::Local> = time.into();
        dt.format("%Y/%m/%d").to_string()
    } else {
        "1970/01/01".to_string()
    }
}
