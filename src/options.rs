use crate::fs::read_file_to_string;

pub fn load_oiff_colors(image_path: &str) -> Vec<String> {
    let file_contents = read_file_to_string(image_path);

    file_contents
        .lines()
        .filter(|line| line.trim().starts_with('#'))
        .map(|line| line.trim().to_string()) // Convert &str to owned String
        .collect()
}

#[derive(Debug, Clone, Copy)]
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
        .unwrap_or_else(|| {
            eprintln!("Failed to load image: Missing \"{target_key}\".");
            std::process::exit(1);
        })
}
