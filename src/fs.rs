use std::{
    env::current_dir,
    fs::{File, OpenOptions, create_dir_all, read_to_string, remove_file},
    io::Write,
    path::Path,
};

use anyhow::Context;

use crate::error;

pub fn get_current_dir() -> String {
    current_dir()
        .unwrap_or_else(|e| error(&format!("Failed to get current directory: {e}")))
        .to_string_lossy()
        .to_string()
}

pub fn path_exists(apparent_path: &str) -> bool {
    let path = expand_home_dir(apparent_path);
    Path::new(&path).exists()
}

pub fn calculate_file_size(bytes: usize) -> String {
    let units = ["Bytes", "KB", "MB", "GB", "TB"];
    if bytes == 0 {
        return "0 Bytes".to_string();
    }

    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{size:.2} {}", units[unit_index])
}

pub fn read_file_to_string(file_path: &str) -> String {
    let path = expand_home_dir(file_path);

    if !path_exists(&path) {
        return String::new();
    }

    read_to_string(&path)
        .unwrap_or_else(|e| error(&format!("Failed to read file: {e}")))
        .replace("\r\n", "\n")
}

pub fn expand_home_dir(apparent_path: &str) -> String {
    apparent_path.replace('~', &std::env::var("HOME").unwrap())
}

pub fn write_to_file(output: &str, contents: &str, append: bool) {
    let path = expand_home_dir(output);
    if let Some(parent_directory) = Path::new(&path).parent() {
        create_dir_all(parent_directory)
            .with_context(|| error(&format!("Failed to create parent directory for: {path}")))
            .unwrap_or_else(|e| error(&format!("Failed to write to file: {e}")));
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
        .with_context(|| error(&format!("Failed to open file: {path}")))
        .unwrap_or_else(|e| error(&format!("Failed to open file: {e}")));

    output_file
        .write_all(contents.as_bytes())
        .with_context(|| error(&format!("Failed to write to file: {path}")))
        .unwrap_or_else(|e| error(&format!("Failed to write to file: {e}")));
}

pub fn create_file(file_to_be_created: &str) {
    let path_of_file = expand_home_dir(file_to_be_created);
    if let Some(parent_folder) = Path::new(&path_of_file).parent() {
        create_dir_all(parent_folder)
            .with_context(|| {
                error(&format!(
                    "Failed to create parent folder for: {path_of_file}"
                ))
            })
            .unwrap_or_else(|e| error(&format!("Failed to create parent folder: {e}")));
    }

    if !path_exists(&path_of_file) {
        File::create(&path_of_file)
            .with_context(|| error(&format!("Failed to create file: {path_of_file}")))
            .unwrap_or_else(|e| error(&format!("Failed to create file: {e}")));
    }
}

pub fn delete_file(path_of_file_to_delete: &str) {
    let path = expand_home_dir(path_of_file_to_delete);
    if path_exists(&path) {
        remove_file(&path)
            .with_context(|| error(&format!("Failed to delete file: {path}")))
            .unwrap_or_else(|e| error(&format!("Failed to delete file: {e}")));
    }
}
