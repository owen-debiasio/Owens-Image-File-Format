use std::io::{Write, stdout};

use crate::{error, fs::read_file_to_string};

pub fn load_oiff_colors(image_path: &str) -> Vec<String> {
    let file_contents = read_file_to_string(image_path);

    let width = get_image_dim(&file_contents, Dimension::Width);
    let height = get_image_dim(&file_contents, Dimension::Height);

    let color_amount = width * height;

    let mut colors = Vec::with_capacity(color_amount);
    let mut last_percent = 0;

    let color_lines = file_contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.starts_with('#'));

    for (index, line) in color_lines.enumerate() {
        let percent = ((index + 1) * 100) / color_amount;

        if percent > last_percent || index == 0 {
            print!("\r\x1B[KReading color: {line} ({percent}%)");
            stdout().flush().unwrap();
            last_percent = percent;
        }

        colors.push(line.to_string());
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
