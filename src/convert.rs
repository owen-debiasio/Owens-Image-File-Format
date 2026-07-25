use std::{
    env::var,
    io::{stdout, Write},
};

use image::{DynamicImage, GenericImageView, ImageError};

use crate::{create_image::create_oiff_image, error, fs::path_exists};

type Image = Result<DynamicImage, ImageError>;

pub fn convert_to_oiff(
    image_path: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !path_exists(image_path) {
        error(&format!("File does not exist: \"{image_path}\""))
    }

    if !image_path.ends_with(".png")
        && !image_path.ends_with(".jpg")
        && !image_path.ends_with(".jpeg")
        && !image_path.ends_with(".webp")
    {
        error("not a supported image file (.png, .jpg, .jpeg, .webp supported)");
    }

    if !output_path.ends_with(".oiff") {
        error(&format!(
            "Incorrect file output name: \"{output_path}\". Must end with \".oiff\"."
        ))
    }

    println!("image: {image_path}");
    println!("output: {output_path}\n");

    let loaded_image = image::open(image_path)?;

    let width = loaded_image.width().try_into().unwrap();
    let height = loaded_image.height().try_into().unwrap();

    let image_colors = convert_colors_to_hex(Ok(loaded_image.clone()), width, height);

    let resolved_output_path = if output_path.is_empty() {
        var("HOME").unwrap_or_else(|_| ".".to_string())
    } else {
        output_path.to_string()
    };

    create_oiff_image(image_colors, width, height, &resolved_output_path);

    Ok(())
}

fn convert_colors_to_hex(image: Image, width: usize, height: usize) -> Vec<String> {
    let color_amount = width * height;
    let mut last_percent = 0;

    let colors = image
        .expect("Failed to load image")
        .pixels()
        .enumerate()
        .map(|(index, (_, _, pixel))| {
            let [r, g, b, a] = pixel.0;
            let color = format!("#{r:02x}{g:02x}{b:02x}{a:02x}");

            let percent = ((index + 1) * 100) / color_amount;

            if percent > last_percent || index == 0 {
                print!("\r\x1B[KConverting color: {color} ({percent}%)");
                stdout().flush().unwrap();
                last_percent = percent;
            }

            color
        })
        .collect();

    println!();

    colors
}
