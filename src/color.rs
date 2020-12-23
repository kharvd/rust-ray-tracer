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
    println!(
        "{} {} {}",
        float_to_int(scaled_col.0),
        float_to_int(scaled_col.1),
        float_to_int(scaled_col.2)
    );
}

pub fn color(r: f64, g: f64, b: f64) -> Color {
    return Vec3(r, g, b);
}