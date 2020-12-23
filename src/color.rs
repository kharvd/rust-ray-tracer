use crate::vec3::{Color, Vec3};

fn float_to_int(v: f64) -> i32 {
    return (256.0 * clamp(v, 0.0, 0.999)) as i32;
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    return if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    };
}

pub fn print_color(col: Color, samples_per_pixel: i32) {
    let scaled_col = col / samples_per_pixel as f64;
    let gamma = 2.0;
    let r = scaled_col.0.powf(1.0 / gamma);
    let g = scaled_col.1.powf(1.0 / gamma);
    let b = scaled_col.2.powf(1.0 / gamma);

    println!(
        "{} {} {}",
        float_to_int(r),
        float_to_int(g),
        float_to_int(b)
    );
}

pub fn color(r: f64, g: f64, b: f64) -> Color {
    return Vec3(r, g, b);
}