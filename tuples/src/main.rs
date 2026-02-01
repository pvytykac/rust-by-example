fn main() {

    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tuple;

    println!("destructured = x: {}, y: {}, z: {}", x, y, z);
    println!("    by index = x: {}, y: {}, z: {}", tuple.0, tuple.1, tuple.2);
}
