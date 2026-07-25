use std::io::{Write, stdout};

use crate::{error, fs::read_file_to_string};

pub fn load_oiff_colors(image_path: &str) -> Vec<String> {
    let file_contents = read_file_to_string(image_path);

    let width = get_image_dim(&file_contents, Dimension::Width);
    let height = get_image_dim(&file_contents, Dimension::Height);

    let total_pixels = width * height;
    let mut colors = Vec::with_capacity(total_pixels);

    let color_lines = file_contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.starts_with('#'));

    for line in color_lines {
        let (hex_str, count) = match line.split_once(':') {
            Some((hex, count_str)) => (hex, count_str.parse::<usize>().unwrap_or(1)),
            None => (line, 1),
        };

        for _ in 0..count {
            colors.push(hex_str.to_string());
        }

        let current = colors.len();
        let percent = ((current * 100) / total_pixels).min(100);
        print!("\r\x1B[KReading colors: {current}/{total_pixels} ({percent}%)");
        stdout().flush().unwrap();
    }

    println!();
    colors
}

pub enum Dimension {
    Width,
    Height,
}

pub fn get_image_dim(image_contents: &str, dim: Dimension) -> usize {
    let target_key = match dim {
        Dimension::Width => "WIDTH",
        Dimension::Height => "HEIGHT",
    };

    image_contents
        .lines()
        .filter_map(|line| line.split_once('='))
        .find_map(|(key, value)| {
            if key.trim().eq_ignore_ascii_case(target_key) {
                value.trim().parse::<usize>().ok()
            } else {
                None
            }
        })
        .unwrap_or_else(|| error("Failed to retrieve image dimensions"))
}
