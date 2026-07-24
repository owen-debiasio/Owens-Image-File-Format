use std::{env::var, io::{Write, stdout}};

use image::{DynamicImage, GenericImageView, ImageError};

use crate::{create_image::create_oiff_image, error, fs::path_exists};

type Image = Result<DynamicImage, ImageError>;

pub fn convert_from_image(
    image_path: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !path_exists(image_path) {
        error(&format!("File does not exist: \"{image_path}\""))
    }
    
    if !image_path.ends_with(".png")
        && !image_path.ends_with(".jpg")
        && !image_path.ends_with(".jpeg")
    {
        error("not a supported image file (.png, .jpg, .jpeg supported)");
    }

    println!("image: {image_path}");
    println!("output: {output_path}\n");

    let loaded_image = image::open(image_path)?;

    let image_colors = convert_colors_to_hex(Ok(loaded_image.clone()));

    let resolved_output_path = if output_path.is_empty() {
        var("HOME").unwrap_or_else(|_| ".".to_string())
    } else {
        output_path.to_string()
    };

    let width = loaded_image.width().try_into().unwrap();
    let height = loaded_image.height().try_into().unwrap();

    create_oiff_image(image_colors, width, height, &resolved_output_path);

    Ok(())
}

fn convert_colors_to_hex(image: Image) -> Vec<String> {
    image
        .expect("")
        .pixels()
        .map(|(_, _, pixel)| {
            let [r, g, b, a] = pixel.0;
            let color = format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a);

            print!("\r\x1B[KConverting color: {color}");
            stdout().flush().unwrap();

            color
        })
        .collect()
}
