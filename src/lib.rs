use wasm_bindgen::prelude::*;
mod image_processing;
use image_processing::image_reader;
use solver::QueensTable;
mod solver;

#[wasm_bindgen]
pub fn run(image_bytes: &[u8]) -> Vec<String> {
    let mut queens_table = QueensTable::from_image(image_bytes);
    queens_table.solve().to_images()

    // Vec::from(image_bytes)
}
