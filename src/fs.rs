use std::{
    fs::{create_dir_all, read_to_string, OpenOptions},
    io::Write,
    path::Path,
};

use anyhow::Context;

pub fn path_exists(apparent_path: &str) -> bool {
    let path = expand_home_dir(apparent_path);
    Path::new(&path).exists()
}

pub fn read_file_to_string(file_path: &str) -> String {
    let path = expand_home_dir(file_path);

    if !path_exists(&path) {
        return String::new();
    }

    read_to_string(&path)
        .unwrap_or_else(|err| panic!("Failed to read file: {err}"))
        .replace("\r\n", "\n")
}

pub fn expand_home_dir(apparent_path: &str) -> String {
    apparent_path.replace('~', &std::env::var("HOME").unwrap())
}

pub fn write_to_file(output: &str, contents: &str, append: bool) {
    let path = expand_home_dir(output);
    if let Some(parent_directory) = Path::new(&path).parent() {
        create_dir_all(parent_directory)
            .with_context(|| format!("Failed to create parent directory for: {path}"))
            .expect("Failed to write to file");
    }

    let mut options = OpenOptions::new();
    options.create(true).write(true);

    if append {
        options.append(true);
    } else {
        options.truncate(true);
    }

    let mut output_file = options
        .open(&path)
        .with_context(|| format!("Failed to open file: {path}"))
        .expect("Failed to get file");

    output_file
        .write_all(contents.as_bytes())
        .with_context(|| format!("Failed writing to file: {path}"))
        .expect("Failed to write contents");
}
