use crate::{error, fs::write_to_file, VERSION};
use std::io::{stdout, Write};

pub fn create_oiff_image(colors: Vec<String>, width: usize, height: usize, output_path: &str) {
    if !output_path.ends_with(".oiff") {
        error(&format!(
            "Incorrect file output name: \"{output_path}\". Must end with \".oiff\"."
        ));
    }

    let color_amount = colors.len();
    let mut last_percent = 0;

    let mut buffer = String::with_capacity(color_amount * 10 + 100);
    buffer.push_str(&format!("Written with OIFF version: {VERSION}\n\n"));
    buffer.push_str(&format!("WIDTH={width}\nHEIGHT={height}\n\n"));

    for (index, color) in colors.iter().enumerate() {
        let percent = ((index + 1) * 100) / color_amount;

        if percent > last_percent || index == 0 {
            print!("\r\x1B[KWriting color: {color} ({percent}%)");
            stdout().flush().unwrap();
            last_percent = percent;
        }

        buffer.push_str(color);
        buffer.push('\n');
    }

    write_to_file(output_path, &buffer, false);

    println!("\n\ncreated image at {output_path}");
}
