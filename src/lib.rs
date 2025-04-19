mod image_reader;
mod solver;

use js_sys::Uint8Array;
use solver::QueensTable;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(image_bytes: &[u8]) -> Vec<JsValue> {
    let mut queens_table = QueensTable::from_image(image_bytes);
    queens_table
        .solve()
        .to_images()
        .iter()
        .map(|img| {
            let uint8array = Uint8Array::new_with_length(img.len() as u32);
            uint8array.copy_from(&img);

            uint8array.into()
        })
        .collect()
}
