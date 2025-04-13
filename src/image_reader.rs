use image::{ImageReader, Rgb};

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

pub fn read_image() {
    // When in the browser, the input will have to be the byte data of the image
    // image::ImageReader::new(BufReader::new())

    // let input_image = ImageReader::open("./assets/examples/queens_3x3_empty.png")
    // let input_image = ImageReader::open("./assets/examples/queens_8x8_empty.png")
    let input_image = ImageReader::open("./assets/examples/queens_10x10_empty.jpg")
        .expect("Failed to open image file")
        .decode()
        .expect("Failed to decode image");
    let width = input_image.width();

    let rgb_image = input_image.into_rgb8();
    let n_hor_divisions = {
        let mut most_hor_divisions = 0;
        let mut current_hor_divisions = 0;

        let mut counter = 0;
        let mut was_previous_black = false;
        for pixel in rgb_image.pixels() {
            if is_black(*pixel) {
                if !was_previous_black {
                    was_previous_black = true;
                    current_hor_divisions += 1;
                    if current_hor_divisions > most_hor_divisions {
                        most_hor_divisions = current_hor_divisions;
                    }
                }
            } else {
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

    
}

// use image::{ImageReader, Rgb};

// fn max_f32(f1: f32, f2: f32) -> f32 {
//     if f1 > f2 {
//         f1
//     } else {
//         f2
//     }
// }
// fn min_f32(f1: f32, f2: f32) -> f32 {
//     if f1 < f2 {
//         f1
//     } else {
//         f2
//     }
// }

// fn rgb_to_hsb(rgb: Rgb<u8>) -> (f32, f32, f32) {
//     let r = rgb.0[0] as f32 / 255.0;
//     let g = rgb.0[1] as f32 / 255.0;
//     let b = rgb.0[2] as f32 / 255.0;

//     if r == 0.0 && g == 0.0 && b == 0.0 {
//         return (0.0, 0.0, 0.0);
//     }

//     let c_max = max_f32(r, max_f32(g, b));
//     let c_min = min_f32(r, min_f32(g, b));

//     let hue = if c_max == c_min {
//         30.0
//     } else {
//         let mut _hue = 0.0;
//         if r == c_max {
//             _hue = 60.0 * ((g - b) / (c_max - c_min));
//         } else if g == c_max {
//             _hue = 60.0 * (2.0 + ((b - r) / (c_max - c_min)));
//         } else {
//             _hue = 60.0 * (4.0 + ((r - g) / (c_max - c_min)));
//         }

//         if _hue < 0.0 {
//             _hue += 360.0;
//         }
//         _hue
//     };

//     let saturation = (c_max - c_min) / c_max;
//     let brightness = c_max;

//     (hue, saturation, brightness)
// }

// fn color_similarity(rgb_1: Rgb<u8>, rgb_2: Rgb<u8>) -> f32 {
//     let (hue_1, saturation_1, brightness_1) = rgb_to_hsb(rgb_1);
//     let (hue_2, saturation_2, brightness_2) = rgb_to_hsb(rgb_2);

//     println!("hue_1: {}\nhue_2: {}", hue_1, hue_2);

//     let hue_distance =// if hue_1 == 0.0 || hue_2 == 0.0 {
//     //     30.0
//     // } else {
//         min_f32(
//             ((hue_1 - hue_2) as f32).abs(),
//             360.0 - ((hue_1 - hue_2) as f32).abs(),
//         );
//     // };
//     let saturation_distance = ((saturation_1 - saturation_2) as f32).abs();
//     let brightness_distance = ((brightness_1 - brightness_2) as f32).abs();

//     println!(
//         "hue diff: {}\nsaturation diff: {}\nbrightness diff: {}",
//         hue_distance, saturation_distance, brightness_distance
//     );

//     // let saturation_weight = if brightness_1 < 0.1 && brightness_2 < 0.1 {
//     //     0.5
//     // } else {
//     //     1.0
//     // };

//     ((1.0 * hue_distance).powf(2.0)
//         + (1.0 * saturation_distance).powf(2.0)
//         + (1.0 * brightness_distance).powf(2.0))
//     .sqrt()
// }

// fn is_black(color: Rgb<u8>) -> bool {
//     if color_similarity(color, Rgb::from([0, 0, 0])) < 30.0 {
//         true
//     } else {
//         false
//     }
// }

// pub fn read_image() {
//     println!(
//         "(147, 147, 149): {}",
//         color_similarity(Rgb::from([147, 147, 149]), Rgb::from([0, 0, 0]))
//     );
//     println!(
//         "(237, 202, 170): {}",
//         color_similarity(Rgb::from([237, 202, 170]), Rgb::from([0, 0, 0]))
//     );
//     println!(
//         "(1, 0, 2): {}",
//         color_similarity(Rgb::from([1, 0, 2]), Rgb::from([0, 0, 0]))
//     );
//     return ();

//     // When in the browser, the input will have to be the byte data of the image
//     // image::ImageReader::new(BufReader::new())

//     // let input_image = ImageReader::open("./assets/examples/queens_3x3_empty.png")
//     let input_image = ImageReader::open("./assets/examples/queens_8x8_empty.png")
//         .expect("Failed to open image file")
//         .decode()
//         .expect("Failed to decode image");
//     let width = input_image.width();
//     // let height = input_image.height();

//     let rgb_image = input_image.into_rgb8();

//     let mut most_hor_divisions = 0;
//     let mut current_hor_divisions = 0;

//     let mut row_counter = 0;

//     let mut counter = 0;
//     // let mut y_counter = 0;
//     let mut was_previous_black = false;
//     for pixel in rgb_image.pixels() {
//         if is_black(*pixel) {
//             if !was_previous_black {
//                 was_previous_black = true;
//                 current_hor_divisions += 1;
//                 if current_hor_divisions > most_hor_divisions {
//                     most_hor_divisions = current_hor_divisions;
//                     println!(
//                         "row: {} | new high: {} | color: {:?}",
//                         row_counter, most_hor_divisions, pixel
//                     );
//                 }
//             }
//         } else {
//             if was_previous_black {
//                 println!("not black anymore{:?}", pixel);
//             }
//             was_previous_black = false;
//         }

//         if counter % width == 0 {
//             row_counter += 1;
//             current_hor_divisions = 0;
//             was_previous_black = false;
//         }

//         counter += 1;
//         // y_counter += 1;
//     }

//     println!("most x divisions: {}", most_hor_divisions);
// }
