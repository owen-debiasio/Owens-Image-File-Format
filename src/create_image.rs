use crate::{VERSION, error, fs::write_to_file};
use std::io::{Write, stdout};

fn calculate_file_size(bytes: usize) -> String {
    let units = ["Bytes", "KB", "MB", "GB", "TB"];
    if bytes == 0 {
        return "0 Bytes".to_string();
    }

    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{size:.2} {}", units[unit_index])
}

pub fn create_oiff_image(colors: Vec<String>, width: usize, height: usize, output_path: &str) {
    if !output_path.ends_with(".oiff") {
        error(&format!(
            "Incorrect file output name: \"{output_path}\". Must end with \".oiff\"."
        ));
    }

    let color_amount = colors.len();

    let width_len = width.to_string().len();
    let height_len = height.to_string().len();

    let estimated_bytes = (color_amount * 10) + 100 + (50 + (width_len + height_len));

    println!(
        "Estimated file size: {}",
        calculate_file_size(estimated_bytes)
    );

    let mut buffer = String::with_capacity(color_amount * 10 + 100);
    buffer.push_str(&format!("Written with OIFF version: {VERSION}\n\n"));
    buffer.push_str(&format!("WIDTH={width}\nHEIGHT={height}\n\n"));

    let mut last_percent = 0;

    for (index, color) in colors.iter().enumerate() {
        let percent = ((index + 1) * 100) / color_amount;

        if percent > last_percent || index == 0 {
            print!(
                "\r\x1B[KWriting color: {} ({percent}%)",
                color
                    .split(':')
                    .next()
                    .unwrap_or_else(|| error("Failed to write color to buffer"))
            );
            stdout().flush().unwrap();
            last_percent = percent;
        }

        buffer.push_str(color);
        buffer.push('\n');
    }

    write_to_file(output_path, &buffer, false);

    println!("\n\nCreated image at \"{output_path}\"");
}
