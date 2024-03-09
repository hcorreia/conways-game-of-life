use base64::{engine::general_purpose::STANDARD, write::EncoderStringWriter};
use image::ImageBuffer;
use std::io::{Cursor, Write};

use crate::life::{BoardState, LIVE};

pub fn draw_image_data_url(state: &BoardState) -> String {
    // TODO: fix the casting. use u32 in BoardState width/height
    let width = state.width as u32;
    let height = state.height as u32;

    let img = ImageBuffer::from_fn(width, height, |x, y| {
        if state.get_index(x as i32, y as i32) == LIVE {
            return image::Rgba([51u8, 51u8, 51u8, 255u8]);
        } else {
            return image::Rgba([204u8, 204u8, 204u8, 255u8]);
        }
    });

    // TODO: Try to make all of this simpler add account for errors:

    let mut bytes: Vec<u8> = Vec::new();
    let mut buf = String::from("data:image/png;base64,");
    let mut encoder = EncoderStringWriter::from_consumer(&mut buf, &STANDARD);

    let mut w = Cursor::new(&mut bytes);

    let _ = img.write_to(&mut w, image::ImageOutputFormat::Png);

    let _ = encoder.write_all(&bytes);
    encoder.into_inner();

    return buf;
}
