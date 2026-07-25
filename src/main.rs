use std::{env::args, process::exit};

use crate::{convert::convert_to_oiff, display::display_oiff_image};

mod convert;
mod create_image;
mod display;
mod fs;
mod options;

fn error(text: &str) -> ! {
    println!("\nERROR: {text}");
    exit(1)
}

fn get_args() -> Vec<String> {
    let mut arg_vec = Vec::new();

    for arg in args().skip(1) {
        arg_vec.push(arg)
    }

    arg_vec
}

pub static VERSION: &str = "v0.1.0";

fn main() {
    let args = get_args();

    let empty = String::new();
    let root_arg = args.first().unwrap_or(&empty);

    match root_arg.as_str() {
        "convert" => {
            let image_path = args.get(1).unwrap_or_else(|| error("Missing image path"));
            let output_file = args.get(2).map(String::as_str).unwrap_or("");

            convert_to_oiff(image_path, output_file).expect("Failed to convert image");
        }

        "display" => {
            let image_path = args.get(1).unwrap_or_else(|| error("Missing image path"));
            display_oiff_image(image_path)
        }

        _ => println!(
            "OIFF {VERSION}\n\
            Commands:\n\n\
            convert <input image path>, <output image path>\n\
            display <image path>"
        ),
    }
}
