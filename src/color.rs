use crate::vec3::Color;

fn float_to_int(v: f64) -> i32 {
    return (255.999 * v) as i32;
}

pub fn print_color(col: Color) {
    println!("{} {} {}", float_to_int(col.0), float_to_int(col.1), float_to_int(col.2));
}