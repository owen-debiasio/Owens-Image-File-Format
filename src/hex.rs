use std::io::{Write, stdout};

use image::{DynamicImage, GenericImageView};

pub fn parse_hex_color(hex: &str) -> [u8; 4] {
    let hex = hex.trim_start_matches('#');

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    let a = if hex.len() >= 8 {
        u8::from_str_radix(&hex[6..8], 16).unwrap_or(255)
    } else {
        255
    };

    [r, g, b, a]
}

pub fn convert_colors_to_hex(image: &DynamicImage, width: usize, height: usize) -> Vec<String> {
    let total_pixels = width * height;
    let update_interval = (total_pixels / 100).max(1);

    println!("Converting colors to hex values...");

    let colors = image
        .pixels()
        .enumerate()
        .map(|(index, (_, _, pixel))| {
            let [r, g, b, a] = pixel.0;
            let color = format!("#{r:02x}{g:02x}{b:02x}{a:02x}");

            if index % update_interval == 0 || index == total_pixels - 1 {
                let percent = ((index + 1) * 100) / total_pixels;
                print!("\r\x1B[KConverting color: {color} ({percent}%)");
                stdout().flush().unwrap();
            }

            color
        })
        .collect();

    println!();
    colors
}
