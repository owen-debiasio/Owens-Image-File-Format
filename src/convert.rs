use std::{env::var, error::Error};

use crate::{
    create_image::create_oiff_image,
    error,
    fs::{path_exists, read_file_to_string},
    hex::{convert_colors_to_hex, parse_hex_color},
    options::{Dimension, get_image_dim, load_oiff_colors},
};
use image::RgbaImage;

fn is_supported_image(path: &str) -> bool {
    path.ends_with(".png")
        || path.ends_with(".jpg")
        || path.ends_with(".jpeg")
        || path.ends_with(".webp")
}

pub fn convert(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    if !path_exists(input_path) {
        error(&format!("File does not exist: \"{input_path}\""));
    }

    let input_lower = input_path.to_lowercase();

    if input_lower.ends_with(".oiff") {
        from_oiff(input_path, output_path)
    } else if is_supported_image(&input_lower) {
        to_oiff(input_path, output_path)
    } else {
        error("Unsupported input format. Must be .png, .jpg, .jpeg, .webp, or .oiff");
    }
}

fn to_oiff(image_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    if !output_path.is_empty() && !output_path.ends_with(".oiff") {
        error(&format!(
            "Incorrect file output name: \"{output_path}\". Must end with \".oiff\"."
        ));
    }

    let resolved_output_path = if output_path.is_empty() {
        format!(
            "{}/output.oiff",
            var("HOME").unwrap_or_else(|_| ".".to_string())
        )
    } else {
        output_path.to_string()
    };

    println!("image: {image_path}");
    println!("output: {resolved_output_path}\n");

    let loaded_image = image::open(image_path)?;
    let width = loaded_image.width().try_into().unwrap();
    let height = loaded_image.height().try_into().unwrap();

    let image_colors = convert_colors_to_hex(&loaded_image, width, height);

    create_oiff_image(image_colors, width, height, &resolved_output_path);

    Ok(())
}

pub fn from_oiff(oiff_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let output_lower = output_path.to_lowercase();
    if !is_supported_image(&output_lower) {
        error("Output format for .oiff must be one of: .png, .jpg, .jpeg, .webp");
    }

    println!("input:  {oiff_path}");
    println!("output: {output_path}\n");

    let file_contents = read_file_to_string(oiff_path);
    let width = get_image_dim(&file_contents, Dimension::Width) as u32;
    let height = get_image_dim(&file_contents, Dimension::Height) as u32;

    let total_pixels = (width * height) as usize;

    println!("Loading .oiff colors...");
    let colors = load_oiff_colors(oiff_path);

    if colors.len() != total_pixels {
        error(&format!(
            "Pixel count mismatch! Expected {total_pixels} pixels ({width}x{height}), but found {}.",
            colors.len()
        ));
    }

    let raw_bytes: Vec<u8> = colors.iter().flat_map(|hex| parse_hex_color(hex)).collect();

    if let Some(img_buffer) = RgbaImage::from_raw(width, height, raw_bytes) {
        println!("Saving image to {output_path}...");
        img_buffer.save(output_path)?;
        println!("Done!");
    } else {
        error("Failed to construct image buffer from raw pixel bytes.");
    }

    Ok(())
}
