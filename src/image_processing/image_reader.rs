use super::super::solver::QueensCell;
use image::Rgb;

fn rgb_to_xyz(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let r = if r <= 0.04045 {
        r / 12.92
    } else {
        ((r + 0.055) / 1.055).powf(2.4)
    };
    let g = if g <= 0.04045 {
        g / 12.92
    } else {
        ((g + 0.055) / 1.055).powf(2.4)
    };
    let b = if b <= 0.04045 {
        b / 12.92
    } else {
        ((b + 0.055) / 1.055).powf(2.4)
    };

    let x = 0.4124564 * r + 0.3575761 * g + 0.1804375 * b;
    let y = 0.2126729 * r + 0.7151522 * g + 0.0721750 * b;
    let z = 0.0193339 * r + 0.1191920 * g + 0.9503041 * b;

    (x, y, z)
}

fn xyz_to_lab(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    let xn = 0.95047;
    let yn = 1.0;
    let zn = 1.08883;

    let x = x / xn;
    let y = y / yn;
    let z = z / zn;

    let x = if x > 0.008856 {
        x.powf(1.0 / 3.0)
    } else {
        7.787 * x + 16.0 / 116.0
    };
    let y = if y > 0.008856 {
        y.powf(1.0 / 3.0)
    } else {
        7.787 * y + 16.0 / 116.0
    };
    let z = if z > 0.008856 {
        z.powf(1.0 / 3.0)
    } else {
        7.787 * z + 16.0 / 116.0
    };

    let l = 116.0 * y - 16.0;
    let a = 500.0 * (x - y);
    let b = 200.0 * (y - z);

    (l, a, b)
}

fn rgb_to_lab(rgb: Rgb<u8>) -> (f32, f32, f32) {
    let (x, y, z) = rgb_to_xyz(rgb.0[0], rgb.0[1], rgb.0[2]);
    xyz_to_lab(x, y, z)
}

fn color_similarity(rgb_1: Rgb<u8>, rgb_2: Rgb<u8>) -> f32 {
    let (lightness_1, a_1, b_1) = rgb_to_lab(rgb_1);
    let (lightness_2, a_2, b_2) = rgb_to_lab(rgb_2);

    let lightness_distance = (lightness_1 - lightness_2).abs();
    let a_distance = (a_1 - a_2).abs();
    let b_distance = (b_1 - b_2).abs();

    ((1.0 * lightness_distance).powf(2.0)
        + (1.0 * a_distance).powf(2.0)
        + (1.0 * b_distance).powf(2.0))
    .sqrt()
}

fn is_black(color: Rgb<u8>) -> bool {
    if color_similarity(color, Rgb::from([0, 0, 0])) < 45.0 {
        true
    } else {
        false
    }
}

pub fn read_image(image_bytes: &[u8]) -> (usize, Vec<Vec<QueensCell>>, Vec<Vec<QueensCell>>) {
    let input_image = image::load_from_memory(image_bytes).expect("Failed to open image file");
    let width = input_image.width() as usize;

    let rgb_image = input_image.clone().into_rgb8();
    let n_hor_divisions = {
        let mut most_hor_divisions = 0;
        let mut current_hor_divisions = 0;

        let mut is_border = false;
        let mut current_border_width = 0;
        let mut border_widths = Vec::new();

        let mut counter = 0;
        let mut was_previous_black = false;
        for pixel in rgb_image.pixels() {
            if is_black(*pixel) {
                if !was_previous_black {
                    is_border = true;
                    current_border_width += 1;

                    was_previous_black = true;
                    current_hor_divisions += 1;
                    if current_hor_divisions > most_hor_divisions {
                        most_hor_divisions = current_hor_divisions;
                    }
                }
            } else {
                if is_border {
                    border_widths.push(current_border_width);
                    current_border_width = 0;
                }
                was_previous_black = false;
            }

            if counter % width == 0 {
                current_hor_divisions = 0;
                was_previous_black = false;
            }

            counter += 1;
        }

        most_hor_divisions
    };
    let n_cells = n_hor_divisions - 1;

    let (cells, cells_by_color) = {
        let cell_width = width / n_cells;

        let mut cells: Vec<Vec<QueensCell>> = Vec::with_capacity(n_cells as usize);
        cells.resize_with(n_cells as usize, || Vec::with_capacity(n_cells as usize));

        let mut cells_by_color: Vec<Vec<QueensCell>> = Vec::with_capacity(n_cells as usize);
        cells_by_color.resize_with(n_cells as usize, || Vec::new());

        for x in 0..n_cells {
            for y in 0..n_cells {
                let x_pos = x * cell_width + cell_width / 2;
                let y_pos = y * cell_width + cell_width / 2;
                let pixel = rgb_image.get_pixel(x_pos as u32, y_pos as u32);

                cells[x as usize].push(QueensCell::new(*pixel, x, y));

                for color_vec in cells_by_color.iter_mut() {
                    if color_vec.len() > 0 {
                        let cell = color_vec
                            .get(0)
                            .expect("Failed to get index 0 of color_vec");
                        if color_similarity(*pixel, cell.color) < 1.0 {
                            color_vec.push(QueensCell::new(*pixel, x, y));
                            break;
                        }
                    } else {
                        color_vec.push(QueensCell::new(*pixel, x, y));
                        break;
                    }
                }
            }
        }

        (cells, cells_by_color)
    };

    (n_cells, cells, cells_by_color)
}
