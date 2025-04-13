use std::ops::DerefMut;

use image::Rgb;

pub struct QueensCell {
    pub color: Rgb<u8>,
    x: u32,
    y: u32,
}
impl QueensCell {
    pub fn new(color: Rgb<u8>, x: u32, y: u32) -> Self {
        Self { color, x, y }
    }
}

pub struct QueensTable {
    width: u32,
    cells: Vec<Vec<QueensCell>>,
    cells_by_color: Vec<Vec<QueensCell>>,
    validation_table: Vec<Vec<bool>>,
}
impl QueensTable {
    pub fn new(
        width: u32,
        cells: Vec<Vec<QueensCell>>,
        cells_by_color: Vec<Vec<QueensCell>>,
    ) -> Self {
        let validation_table: Vec<Vec<bool>> = {
            let mut validation_table = Vec::with_capacity(width as usize);
            validation_table.resize_with(width as usize, || {
                let mut vec = Vec::with_capacity(width as usize);
                vec.resize_with(width as usize, || false);

                vec
            });

            validation_table
        };

        let mut table = Self {
            width,
            cells,
            cells_by_color,
            validation_table,
        };

        table.cells_by_color.sort_by(|a, b| a.len().cmp(&b.len()));

        table
    }

}
