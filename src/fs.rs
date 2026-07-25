use std::{
    fs::{create_dir_all, read_to_string, remove_file, File, OpenOptions},
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

pub fn create_file(file_to_be_created: &str) {
    let path_of_file = expand_home_dir(file_to_be_created);
    if let Some(parent_folder) = Path::new(&path_of_file).parent() {
        create_dir_all(parent_folder)
            .with_context(|| format!("Failed to create parent folder for: {path_of_file}"))
            .expect("Failed to create parent folder");
    }

    if !path_exists(&path_of_file) {
        File::create(&path_of_file)
            .with_context(|| format!("Failed to create file: {path_of_file}"))
            .expect("Failed to create file");
    }
}

pub fn delete_file(path_of_file_to_delete: &str) {
    let path = expand_home_dir(path_of_file_to_delete);
    if path_exists(&path) {
        remove_file(&path)
            .with_context(|| format!("Failed to delete file: {path}"))
            .expect("Failed to delete file");
    }
}
