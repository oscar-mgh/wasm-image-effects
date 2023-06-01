use base64::{decode, encode};
use image::ImageOutputFormat::Png;
use image::{load_from_memory, DynamicImage};
use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;

fn create_image(encoded_file: &str) -> DynamicImage {
    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());

    return img;
}

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    let mut img = create_image(&encoded_file);

    img = img.grayscale();
    log(&"effect applied".into());

    let mut buffer = vec![];
    img.write_to(&mut buffer, Png).unwrap();
    log(&"New image written".into());

    let encoded_image = encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", encoded_image);

    return data_url;
}
