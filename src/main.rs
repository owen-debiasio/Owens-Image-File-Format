use std::{env::args, process::exit};

use crate::{convert::convert, display::display_oiff_image, thumbnailer::generate_thumb};

mod convert;
mod create_image;
mod display;
mod fs;
mod hex;
mod options;
mod thumbnailer;

pub static VERSION: &str = "v0.3.1";

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

fn main() {
    let args = get_args();

    let empty = String::new();
    let root_arg = args.first().unwrap_or(&empty);

    match root_arg.as_str() {
        "convert" => {
            let image_path = args.get(1).unwrap_or_else(|| error("Missing image path"));
            let output_file = args.get(2).map(String::as_str).unwrap_or("");

            convert(image_path, output_file)
                .unwrap_or_else(|e| error(&format!("Failed to convert image: {e}")));
        }

        "display" => {
            let image_path = args.get(1).unwrap_or_else(|| error("Missing image path"));
            display_oiff_image(image_path)
        }

        "thumbnail" => {
            let input_path = args
                .get(1)
                .unwrap_or_else(|| error("Missing input image path"));
            let output_path = args
                .get(2)
                .unwrap_or_else(|| error("Missing output image path"));
            let size = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(128);

            generate_thumb(input_path, output_path, size)
                .unwrap_or_else(|e| error(&format!("Failed to generate thumbnail: {e}")));
        }

        "--version" => println!("oiff {VERSION}"),

        _ => println!(
            "OIFF {VERSION}\n\n\
            Commands:\n\n\
            convert <input image path>, <output image path>\n\
            display <.oiff image path>\n\
            thumbnail <input image> <output image> <size (default: 128px)>\n\n\
            Flags:\n\n\
            --version"
        ),
    }
}
