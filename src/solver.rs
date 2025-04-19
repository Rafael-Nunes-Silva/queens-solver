use image::{
    codecs::gif::{GifEncoder, Repeat},
    DynamicImage, Frame, GenericImage, GenericImageView, ImageFormat, Rgb, Rgba,
};
use std::io::Cursor;

struct ValidationTable {
    cells: Vec<Vec<bool>>,
}
impl ValidationTable {
    fn new(width: usize) -> Self {
        let cells: Vec<Vec<bool>> = {
            let mut table = Vec::with_capacity(width);
            table.resize_with(width, || {
                let mut vec = Vec::with_capacity(width);
                vec.resize_with(width, || false);

                vec
            });

            table
        };
        ValidationTable { cells }
    }

    fn validate(&self) -> bool {
        for cells in &self.cells {
            for cell in cells {
                if !*cell {
                    return false;
                }
            }
        }
        true
    }

    fn validate_cell(&self, cell: &QueensCell) -> bool {
        !*self
            .cells
            .get(cell.y as usize)
            .expect("Failed to get cell.y from validation_table")
            .get(cell.x as usize)
            .expect("Failed to get cell.x from validation_table")
    }

    fn play_cell(&mut self, x: usize, y: usize) -> Self {
        Self {
            cells: self
                .cells
                .iter()
                .enumerate()
                .map(|cells| {
                    cells
                        .1
                        .iter()
                        .enumerate()
                        .map(move |cell| {
                            if cell.0 == x as usize || cells.0 == y as usize {
                                return true;
                            }
                            if cell.0.abs_diff(x as usize) <= 1 && cells.0.abs_diff(y as usize) <= 1
                            {
                                return true;
                            }
                            return *cell.1;
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

#[derive(Clone)]
pub struct QueensCell {
    pub color: Rgb<u8>,
    x: usize,
    y: usize,
}
impl QueensCell {
    pub fn new(color: Rgb<u8>, x: usize, y: usize) -> Self {
        Self { color, x, y }
    }
}

pub struct QueensTable {
    width: usize,
    cells: Vec<Vec<QueensCell>>,
    cells_by_color: Vec<Vec<QueensCell>>,
    played_positions: Vec<(usize, usize)>,
    validation_tables: Vec<ValidationTable>,
}
impl QueensTable {
    pub fn new(
        width: usize,
        cells: Vec<Vec<QueensCell>>,
        cells_by_color: Vec<Vec<QueensCell>>,
    ) -> Self {
        let mut table = Self {
            width,
            cells,
            cells_by_color,
            played_positions: Vec::new(),
            validation_tables: Vec::new(),
        };

        table.cells_by_color.sort_by(|a, b| a.len().cmp(&b.len()));

        table
    }

    pub fn from_image(image_bytes: &[u8]) -> Self {
        let (width, cells, cells_by_color) = super::image_reader::read_image(image_bytes);
        QueensTable::new(width, cells, cells_by_color)
    }

    pub fn solve(&mut self) -> &mut Self {
        let mut color_index: usize = 0;
        let mut cell_index_stack: Vec<usize> = Vec::with_capacity(self.cells_by_color.len());
        cell_index_stack.resize_with(self.cells_by_color.len(), || 0);

        let n_validation_tables = self.cells_by_color.len() + 1;
        let mut validation_tables: Vec<ValidationTable> = Vec::with_capacity(n_validation_tables);
        validation_tables.resize_with(n_validation_tables, || {
            ValidationTable::new(self.width as usize)
        });

        let mut played_positions = Vec::new();
        while !validation_tables[color_index].validate() {
            let cells = self.cells_by_color[color_index].clone();

            let validation_table = validation_tables
                .get_mut(color_index)
                .expect("Failed to get color_index from validation_tables");

            let mut played = false;
            while cell_index_stack[color_index] < cells.len() {
                let cell = &cells[cell_index_stack[color_index]];
                let x = cell.x;
                let y = cell.y;

                if validation_table.validate_cell(cell) {
                    played = true;
                    color_index += 1;

                    played_positions.push((x, y));

                    let updated_validation_table = validation_table.play_cell(cell.x, cell.y);

                    validation_tables[color_index] = updated_validation_table;
                    break;
                }
                cell_index_stack[if played { color_index - 1 } else { color_index }] += 1;
            }
            if !played {
                played_positions.pop();
                cell_index_stack[color_index] = 0;
                if color_index == 0 {
                    panic!("Could not solve queens");
                }
                color_index -= 1;
                cell_index_stack[color_index] += 1;
            }
        }

        self.played_positions = played_positions;
        self.validation_tables = validation_tables;

        self
    }

    fn to_image(&self, table: &ValidationTable) -> Vec<u8> {
        let division_size: u32 = 5;
        let cell_size: u32 = 90;
        let total_image_size: u32 =
            (self.width as u32 * cell_size) + ((self.width as u32 + 1) * division_size);

        let mut image = DynamicImage::new_rgb8(total_image_size, total_image_size);

        let crown_bytes: &[u8] = include_bytes!("../assets/crown.png");
        let crown_image =
            image::load_from_memory(crown_bytes).expect("Failed to load the crown image");

        let cross_bytes: &[u8] = include_bytes!("../assets/cross.png");
        let cross_image =
            image::load_from_memory(cross_bytes).expect("Failed to load the cross image");

        for (row_index, row) in self.cells.iter().enumerate() {
            for (cell_index, cell) in row.iter().enumerate() {
                for x in 0..cell_size {
                    for y in 0..cell_size {
                        let x_offset = x
                            + cell_index as u32 * cell_size
                            + (cell_index as u32 + 1) * division_size;

                        let y_offset = y
                            + row_index as u32 * cell_size
                            + (row_index as u32 + 1) * division_size;

                        if !table.cells[cell_index][row_index] {
                            image.put_pixel(
                                y_offset,
                                x_offset,
                                Rgba([cell.color[0], cell.color[1], cell.color[2], 255]),
                            );
                            continue;
                        }

                        let pixel_color = {
                            if self.played_positions.contains(&(row_index, cell_index)) {
                                let crown_pixel = crown_image.get_pixel(y, x);
                                let crown_alpha: f32 = crown_pixel[3] as f32 / 255.0;

                                Rgba([
                                    (crown_pixel[0] as f32 * crown_alpha
                                        + cell.color[0] as f32 * (1.0 - crown_alpha))
                                        as u8,
                                    (crown_pixel[1] as f32 * crown_alpha
                                        + cell.color[1] as f32 * (1.0 - crown_alpha))
                                        as u8,
                                    (crown_pixel[2] as f32 * crown_alpha
                                        + cell.color[2] as f32 * (1.0 - crown_alpha))
                                        as u8,
                                    255,
                                ])
                            } else {
                                let cross_pixel = cross_image.get_pixel(y, x);
                                let cross_alpha: f32 = cross_pixel[3] as f32 / 255.0;

                                Rgba([
                                    (cross_pixel[0] as f32 * cross_alpha
                                        + cell.color[0] as f32 * (1.0 - cross_alpha))
                                        as u8,
                                    (cross_pixel[1] as f32 * cross_alpha
                                        + cell.color[1] as f32 * (1.0 - cross_alpha))
                                        as u8,
                                    (cross_pixel[2] as f32 * cross_alpha
                                        + cell.color[2] as f32 * (1.0 - cross_alpha))
                                        as u8,
                                    255,
                                ])
                            }
                        };

                        image.put_pixel(y_offset, x_offset, pixel_color);
                    }
                }
            }
        }

        let mut bytes: Vec<u8> = Vec::new();
        image
            .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
            .unwrap();

        bytes
    }

    pub fn to_images(&self) -> Vec<Vec<u8>> {
        let mut images = Vec::with_capacity(self.validation_tables.len());
        for table in &self.validation_tables {
            images.push(self.to_image(table));
        }
        images
    }

    pub fn to_gif(&self) -> Vec<u8> {
        let mut gif_bytes: Vec<u8> = Vec::new();

        {
            let mut writer = Cursor::new(&mut gif_bytes);
            let mut encoder = GifEncoder::new_with_speed(&mut writer, 10);

            encoder
                .set_repeat(Repeat::Infinite)
                .expect("Failed to set repeat behaviour");

            for image in self.to_images() {
                let frame = Frame::new(
                    image::load_from_memory(&image)
                        .expect("Failed to load image from memory")
                        .to_rgba8(),
                );
                encoder.encode_frame(frame).expect("Failed to encode frame");
            }
        }

        gif_bytes
    }
}
