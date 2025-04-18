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

    fn play_cell(&self, x: u32, y: u32) -> Self {
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
    cells_by_color: Vec<Vec<QueensCell>>,
    // validation_table: Vec<Vec<bool>>,
}
impl QueensTable {
    pub fn new(width: u32, cells_by_color: Vec<Vec<QueensCell>>) -> Self {
        let mut table = Self {
            width,
            cells_by_color,
        };

        table.cells_by_color.sort_by(|a, b| a.len().cmp(&b.len()));

        table
    }

    pub fn solve(&mut self) {
        let mut color_index: usize = 0;
        let mut cell_index_stack: Vec<usize> = Vec::with_capacity(self.cells_by_color.len());
        cell_index_stack.resize_with(self.cells_by_color.len(), || 0);

        let n_validation_tables = self.cells_by_color.len() + 1;
        let mut validation_tables: Vec<ValidationTable> = Vec::with_capacity(n_validation_tables);
        validation_tables.resize_with(n_validation_tables, || {
            ValidationTable::new(self.width as usize)
        });

        while !validation_tables[color_index].validate() {
            println!(
                "color_index: {}\nindex_stack: {}",
                color_index, cell_index_stack[color_index]
            );

            let cells = self.cells_by_color[color_index].clone();

            let validation_table = &validation_tables[color_index];

            let mut played = false;
            while cell_index_stack[color_index] < cells.len() {
                let cell = &cells[cell_index_stack[color_index]];
                let x = cell.x;
                let y = cell.y;

                if validation_table.validate_cell(cell) {
                    println!("Played at: {}, {}", x, y);

                    played = true;
                    color_index += 1;

                    let updated_validation_table = validation_table.play_cell(cell.x, cell.y);

                    for row in &updated_validation_table.cells {
                        let row_str = row
                            .iter()
                            .map(|c| if *c { "+" } else { "#" })
                            .collect::<Vec<&str>>()
                            .join(" ");
                        println!("{}", row_str);
                    }

                    validation_tables[color_index] = updated_validation_table;
                    break;
                }
                println!("Skipping: {}, {}", x, y);
                cell_index_stack[if played { color_index - 1 } else { color_index }] += 1;
            }
            if !played {
                println!("End, has to backtrack from here");
                cell_index_stack[color_index] = 0;
                if color_index == 0 {
                    println!("THIS SHOULD NEVER BE TRUE");
                    break;
                }
                color_index -= 1;
                cell_index_stack[color_index] += 1;
            }
        }

        for cell_list in &self.cells_by_color {
            println!("{}", cell_list.len());
        }
    }
}
