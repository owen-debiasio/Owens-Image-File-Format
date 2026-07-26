use std::io::{Write, stdout};

use minifb::{Window, WindowOptions};

use crate::{
    error,
    fs::{path_exists, read_file_to_string},
    options::{Dimension, get_image_dim, load_oiff_colors},
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

    let mut buffer = vec![0x00000000; width * height];

    println!("Initializing colors...");

    let colors = load_oiff_colors(path);
    let total = width * height;

    if colors.len() != total {
        error(&format!(
            "Color count mismatch! Expected {total} pixels ({width}x{height}), but got {}.",
            colors.len()
        ));
    }

    let mut last_percent = 0;

    for (color_index, color_str) in colors.iter().enumerate().take(total) {
        let percent = ((color_index + 1) * 100) / total;

        if percent > last_percent || color_index == 0 {
            print!("\r\x1B[KLoading color: {color_str} ({percent}%)");
            stdout().flush().unwrap();
            last_percent = percent;
        }

        let clean_hex = color_str.trim_start_matches('#');

        let pixel_color = match clean_hex.len() {
            8 => {
                let alpha = u32::from_str_radix(&clean_hex[6..8], 16).unwrap_or(255);

                if alpha == 0 {
                    0x00000000
                } else {
                    let r = u32::from_str_radix(&clean_hex[0..2], 16).unwrap_or(0);
                    let g = u32::from_str_radix(&clean_hex[2..4], 16).unwrap_or(0);
                    let b = u32::from_str_radix(&clean_hex[4..6], 16).unwrap_or(0);
                    (r << 16) | (g << 8) | b
                }
            }
            6 => u32::from_str_radix(clean_hex, 16).unwrap_or(0x00000000),
            _ => 0x00000000,
        };

        buffer[color_index] = pixel_color;
    }

    println!();

    let max_image_dim = width.max(height) as f32;

    let min_target_dim = 400.0;
    let max_target_dim = 800.0;

    let target_dim = if max_image_dim < min_target_dim {
        min_target_dim
    } else if max_image_dim > max_target_dim {
        max_target_dim
    } else {
        max_image_dim
    };

    let scale_factor = target_dim / max_image_dim;

    let rendered_width = ((width as f32) * scale_factor).round() as usize;
    let rendered_height = ((height as f32) * scale_factor).round() as usize;

    println!(
        "Rendered window dimensions: {rendered_width}x{rendered_height} (Scale: {scale_factor:.2}x)"
    );

    println!("Initializing window...");

    let mut window = Window::new(
        path,
        rendered_width,
        rendered_height,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        error(&format!("Failed to display image: {e}"));
    });

    println!("Starting window...");

    window
        .update_with_buffer(&buffer, width, height)
        .unwrap_or_else(|e| error(&format!("Failed to draw initial frame: {e}")));

    while window.is_open() {
        window.update();
    }
}
