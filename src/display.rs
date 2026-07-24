use minifb::{Key, ScaleMode, Window, WindowOptions};

use crate::{
    error,
    fs::{path_exists, read_file_to_string},
    options::{get_image_dim, load_oiff_colors, Dimension},
};

pub fn display_oiff_image(path: &str) {
    println!("Loading image: \"{path}\"...");

    if !path_exists(path) {
        error(&format!("File does not exist: \"{path}\""))
    }

    if !path.ends_with(".oiff") {
        error("Unsupported image format")
    }

    let file_contents = read_file_to_string(path);

    let width = get_image_dim(&file_contents, Dimension::Width);
    let height = get_image_dim(&file_contents, Dimension::Height);

    println!("Loaded image dimensions: {width}x{height}");

    let mut buffer = vec![0x00FFFFFF; width * height];

    println!("Initializing colors...");

    let colors = load_oiff_colors(path);

    for (color_index, color_str) in colors.iter().enumerate().take(width * height) {
        let clean_hex = color_str.trim_start_matches('#');

        if let Some(pixel_color) = clean_hex
            .get(..6)
            .and_then(|color_string| u32::from_str_radix(color_string, 16).ok())
        {
            buffer[color_index] = pixel_color;
        }
    }

    let rendered_width = width / 2;
    let rendered_height = height / 2;
    println!("Rendered image dimensions: {rendered_width}x{rendered_height}");

    println!("Initializing window...");

    let mut window = Window::new(
        path,
        rendered_width,
        rendered_height,
        WindowOptions {
            resize: false,
            scale_mode: ScaleMode::AspectRatioStretch,
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
