use image::Rgb;

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

    fn play_cell(&self, x: usize, y: usize) -> Self {
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

            let validation_table = &validation_tables[color_index];

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

    pub fn to_images(&self) -> Vec<String> {
        let mut strs = Vec::new();
        for row in &self.cells {
            let mut str = String::new();
            for cell in row {
                if self.played_positions.contains(&(cell.y, cell.x)) {
                    str.push_str("@ ");
                } else {
                    str.push_str("# ");
                }
            }
            strs.push(str);
        }
        strs
    }

    pub fn to_gif() {}
}
