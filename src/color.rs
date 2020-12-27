use std::{ops, f64};

use image::{Rgb, RgbImage, ImageBuffer};
use rand::{Rng, RngCore};
use serde::{Deserialize, Serialize};

fn float_to_int(v: f64) -> u8 {
    return (256.0 * clamp(v, 0.0, 0.999)) as u8;
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

fn map_color_value(x: f64, samples_per_pixel: u32) -> u8 {
    let gamma = 2.0;
    float_to_int((x / samples_per_pixel as f64).powf(1.0 / gamma))
}

pub(crate) fn get_rgb_u8(pix: Rgb<f64>, samples_per_pixel: u32) -> Rgb<u8> {
    let r = map_color_value(pix.0[0], samples_per_pixel);
    let g = map_color_value(pix.0[1], samples_per_pixel);
    let b = map_color_value(pix.0[2], samples_per_pixel);
    Rgb([r, g, b])
}

pub fn discretize_image(buf: &ImageBuffer<Rgb<f64>, Vec<f64>>, samples_per_pixel: u32) -> RgbImage {
    ImageBuffer::from_fn(
        buf.width(),
        buf.height(),
        |x, y| get_rgb_u8(buf[(x, y)], samples_per_pixel)
    )
}

pub fn update_pixel(buf: &mut ImageBuffer<Rgb<f64>, Vec<f64>>, x: u32, y: u32, color: Color) {
    buf[(x, y)].0[0] += color.r;
    buf[(x, y)].0[1] += color.g;
    buf[(x, y)].0[2] += color.b;
}
