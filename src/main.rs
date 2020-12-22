mod vec3;
mod color;

use vec3::Vec3;
use crate::color::print_color;

fn main() {
    let image_width = 256;
    let image_height = 256;
    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let pix = Vec3(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25
            );

            print_color(pix);
        }
    }
}
