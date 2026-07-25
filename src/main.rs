use std::{env::args, process::exit};

use crate::{
    convert::convert, display::display_oiff_image, gui::picker::open_oiff,
    thumbnailer::generate_thumb,
};

mod convert;
mod create_image;
mod display;
mod fs;
mod hex;
mod options;

mod gui;

// This is a hidden command where the user never needs to generate a thumbnail
mod thumbnailer;

fn error(text: &str) -> ! {
    eprintln!("\nERROR: {text}");
    exit(1)
}

fn get_args() -> Vec<String> {
    let mut arg_vec = Vec::new();

    for arg in args().skip(1) {
        arg_vec.push(arg)
    }

    arg_vec
}

pub static VERSION: &str = "v0.2.0";

fn main() {
    let args = get_args();

    let empty = String::new();
    let root_arg = args.first().unwrap_or(&empty);

    match root_arg.as_str() {
        "convert" => {
            let image_path = args.get(1).unwrap_or_else(|| error("Missing image path"));
            let output_file = args.get(2).map(String::as_str).unwrap_or("");

            convert(image_path, output_file).expect("Failed to convert image");
        }

        "display" => {
            let image_path = args.get(1).unwrap_or_else(|| error("Missing image path"));
            display_oiff_image(image_path)
        }

        // This is a hidden command where the user never needs to generate a thumbnail
        "thumbnail" => {
            let input_path = args
                .get(1)
                .unwrap_or_else(|| error("Missing input image path"));
            let output_path = args
                .get(2)
                .unwrap_or_else(|| error("Missing output image path"));
            let size = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(128);

            generate_thumb(input_path, output_path, size).expect("Failed to generate thumbnail");
        }

        // Also a hidden command
        "open_oiff" => open_oiff(),

        _ => println!(
            "OIFF {VERSION}\n\
            Commands:\n\n\
            convert <input image path>, <output image path>\n\
            display <image path>"
        ),
    }
}
