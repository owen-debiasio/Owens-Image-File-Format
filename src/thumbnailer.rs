use std::{error::Error, process::id as process_id};

use crate::{
    convert::from_oiff,
    error,
    fs::{create_file, delete_file, expand_home_dir, path_exists},
};

pub fn generate_thumb(oiff_path: &str, output_path: &str, size: u32) -> Result<(), Box<dyn Error>> {
    if !path_exists(oiff_path) {
        error(&format!("Input file does not exist: \"{oiff_path}\""))
    }

    let temp_png_path = format!("/tmp/oiff_preview_{}.png", process_id());

    from_oiff(oiff_path, &temp_png_path).unwrap_or_else(|e| {
        error(&format!(
            "Failed to convert from oiff file \"{oiff_path}\" to \"{temp_png_path}\": {e}"
        ))
    });

    let img = image::open(&temp_png_path)
        .unwrap_or_else(|e| error(&format!("Failed to open image \"{temp_png_path}\": {e}")));
    let thumbnail = img.thumbnail(size, size);

    create_file(output_path);

    thumbnail
        .save(expand_home_dir(output_path))
        .unwrap_or_else(|e| error(&format!("Failed to save thumbnail \"{output_path}\": {e}")));

    delete_file(&temp_png_path);

    Ok(())
}
