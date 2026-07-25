use std::{error::Error, process::id};

use crate::{
    convert::convert_from_oiff,
    fs::{create_file, delete_file, expand_home_dir, path_exists},
};

pub fn generate_thumb(oiff_path: &str, output_path: &str, size: u32) -> Result<(), Box<dyn Error>> {
    if !path_exists(oiff_path) {
        return Err(format!("Input file does not exist: \"{oiff_path}\"").into());
    }

    let temp_png_path = format!("/tmp/oiff_preview_{}.png", id());

    convert_from_oiff(oiff_path, &temp_png_path)?;

    let img = image::open(&temp_png_path)?;
    let thumbnail = img.thumbnail(size, size);

    create_file(output_path);

    thumbnail.save(expand_home_dir(output_path))?;

    delete_file(&temp_png_path);

    Ok(())
}
