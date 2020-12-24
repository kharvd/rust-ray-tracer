use rand::{RngCore, Rng};
use std::ops;
use serde::{Serialize, Deserialize};

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        return Color { r, g, b };
    }

    pub fn random(rng: &mut dyn RngCore) -> Color {
        return Color::new(rng.gen(), rng.gen(), rng.gen());
    }
}

impl_op_ex!(+ |lhs: &Color, rhs: &Color| -> Color { Color::new(lhs.r + rhs.r, lhs.g + rhs.g, lhs.b + rhs.b) });
impl_op_ex!(+= |lhs: &mut Color, rhs: &Color| { lhs.r += rhs.r; lhs.g += rhs.g; lhs.b += rhs.b });
impl_op_ex_commutative!(+ |lhs: &Color, rhs: f64| -> Color { Color::new(lhs.r + rhs, lhs.g + rhs, lhs.b + rhs) });

impl_op_ex!(* |lhs: &Color, rhs: &Color| -> Color { Color::new(lhs.r * rhs.r, lhs.g * rhs.g, lhs.b * rhs.b) });
impl_op_ex_commutative!(* |lhs: &Color, rhs: f64| -> Color { Color::new(lhs.r * rhs, lhs.g * rhs, lhs.b * rhs) });

impl_op_ex!(/ |lhs: &Color, rhs: f64| -> Color { Color::new(lhs.r / rhs, lhs.g / rhs, lhs.b / rhs) });
impl_op_ex!(/ |lhs: f64, rhs: &Color| -> Color { Color::new(lhs / rhs.r, lhs / rhs.g, lhs / rhs.b) });

pub fn print_color(col: Color, samples_per_pixel: i32) {
    let scaled_col = col / samples_per_pixel as f64;
    let gamma = 2.0;
    let r = scaled_col.r.powf(1.0 / gamma);
    let g = scaled_col.g.powf(1.0 / gamma);
    let b = scaled_col.b.powf(1.0 / gamma);

    println!(
        "{} {} {}",
        float_to_int(r),
        float_to_int(g),
        float_to_int(b)
    );
}