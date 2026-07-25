use std::io::{stdout, Write};

use minifb::{Key, Window, WindowOptions};

use crate::{
    error,
    fs::{path_exists, read_file_to_string},
    options::{get_image_dim, load_oiff_colors, Dimension},
};

pub fn display_oiff_image(path: &str) {
    println!("Loading image: \"{path}\"...");

    if !path_exists(path) {
        error(&format!("File does not exist: \"{path}\""));
    }

    if !path.ends_with(".oiff") {
        error("Please load a \".oiff\" file.");
    }

    let file_contents = read_file_to_string(path);

    let width = get_image_dim(&file_contents, Dimension::Width);
    let height = get_image_dim(&file_contents, Dimension::Height);

    println!("Loaded image dimensions: {width}x{height}");

    let mut buffer = vec![0x00FFFFFF; width * height];

    println!("Initializing colors...");

    let colors = load_oiff_colors(path);
    let total = width * height;
    let mut last_percent = 0;

    for (color_index, color_str) in colors.iter().enumerate().take(total) {
        let percent = ((color_index + 1) * 100) / total;

        if percent > last_percent || color_index == 0 {
            print!("\r\x1B[KLoading color: {color_str} ({percent}%)");
            stdout().flush().unwrap();
            last_percent = percent;
        }

        let clean_hex = color_str.trim_start_matches('#');

        if let Some(pixel_color) = clean_hex
            .get(..6)
            .and_then(|color_string| u32::from_str_radix(color_string, 16).ok())
        {
            buffer[color_index] = pixel_color;
        }
    }

    println!();

    let max_target_dim = 800.0;
    let max_image_dim = width.max(height) as f32;
    let scale_factor = (max_target_dim / max_image_dim).min(1.0);

    let rendered_width = ((width as f32) * scale_factor).round() as usize;
    let rendered_height = ((height as f32) * scale_factor).round() as usize;

    println!("Rendered window dimensions: {rendered_width}x{rendered_height}");

    println!("Initializing window...");

    let mut window = Window::new(
        path,
        rendered_width,
        rendered_height,
        WindowOptions {
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("Failed to display image: {e}");
    });

    println!("Starting window...");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, width, height)
            .unwrap_or_else(|error| {
                panic!("Failed to update window: {error}");
            });
    }
}
