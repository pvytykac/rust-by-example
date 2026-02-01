fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    let mut y = x;
    y += 1;
    println!("The value of y is: {}", y);

    let z: i64 = 64;
    println!("The value of z is: {}", z);

    let z: &str = "redeclared";
    println!("The value of z is now: {}", z);
}
