fn float_to_int(v: f64) -> i32 {
    return (255.999 * v) as i32;
}

fn main() {
    let image_width = 256;
    let image_height = 256;
    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let ir = float_to_int(r);
            let ig = float_to_int(g);
            let ib = float_to_int(b);
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
