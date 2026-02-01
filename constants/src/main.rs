const TEN: u32 = 10;

fn main() {
    const TWENTY: u32 = 20;
    const TUPLE: (u32, u32) = (TEN, TWENTY);
    const ARRAY: [u32; 2] = [TEN, TWENTY];

    println!("GLOBAL: '{}'", TEN);
    println!("ONE: '{}'", TWENTY);
    println!("TUPLE: '{:?}'", TUPLE);
    println!("ARRAY: '{:?}'", ARRAY);
}
