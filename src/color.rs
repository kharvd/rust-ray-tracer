use crate::vec3::Vec3;

fn float_to_int(v: f64) -> i32 {
    return (255.999 * v) as i32;
}

pub fn print_color(vec: Vec3) {
    println!("{} {} {}", float_to_int(vec.0), float_to_int(vec.1), float_to_int(vec.2));
}