use std::io::Write;

use crate::{error, fs::write_to_file, VERSION};

pub fn create_oiff_image(colors: Vec<String>, width: usize, height: usize, output_path: &str) {
    if !output_path.ends_with(".oiff") {
        error(&format!(
            "Incorrect file output name: \"{output_path}\". Must end with \".oiff\"."
        ))
    }

    write_to_file(
        output_path,
        &format!("Written with OIFF version: {VERSION}\n\n"),
        false,
    );

    write_to_file(
        output_path,
        &format!("WIDTH={width}\nHEIGHT={height}\n\n"),
        true,
    );

    let color_amount = colors.len();

    for (index, color) in colors.iter().enumerate() {
        let percent = ((index + 1) * 100) / color_amount;

        print!("\r\x1B[KWriting color: {color} ({percent}%)");
        std::io::stdout().flush().unwrap();

        write_to_file(output_path, &format!("{color}\n"), true);
    }

    println!("\n\ncreated image at {output_path}")
}
