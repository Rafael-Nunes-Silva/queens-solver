mod image_reader;
mod solver;

pub fn run() {
    let queens_table = image_reader::read_image();
    // queens_table.solve();
}
