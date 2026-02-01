extern crate core;

use std::fmt::Display;

fn main() {
    struct ScalarRange<T: Display> {
        type_name: String,
        min: T,
        max: T,
    }

    impl<T: Display> ScalarRange<T> {
        fn print_range(&self) {
            println!("{} range = <{}, {}>", self.type_name, self.min, self.max);
        }
    }

    // i
    ScalarRange {
        type_name: String::from("  i8"),
        min: i8::MIN,
        max: i8::MAX,
    }
    .print_range();
    ScalarRange {
        type_name: String::from(" i16"),
        min: i16::MIN,
        max: i16::MAX,
    }
    .print_range();
    ScalarRange {
        type_name: String::from(" i32"),
        min: i32::MIN,
        max: i32::MAX,
    }
    .print_range();
    ScalarRange {
        type_name: String::from(" i64"),
        min: i64::MIN,
        max: i64::MAX,
    }
    .print_range();
    ScalarRange {
        type_name: String::from("i128"),
        min: i128::MIN,
        max: i128::MAX,
    }
    .print_range();

    // u
    ScalarRange {
        type_name: String::from("  u8"),
        min: u8::MIN,
        max: u8::MAX,
    }
    .print_range();
    ScalarRange {
        type_name: String::from(" u16"),
        min: u16::MIN,
        max: u16::MAX,
    }
    .print_range();
    ScalarRange {
        type_name: String::from(" u32"),
        min: u32::MIN,
        max: u32::MAX,
    }
    .print_range();
    ScalarRange {
        type_name: String::from(" u64"),
        min: u64::MIN,
        max: u64::MAX,
    }
    .print_range();
    ScalarRange {
        type_name: String::from("u128"),
        min: u128::MIN,
        max: u128::MAX,
    }
    .print_range();

    // f
    ScalarRange {
        type_name: String::from("f32"),
        min: f32::MIN,
        max: f32::MAX,
    }
    .print_range();
    ScalarRange {
        type_name: String::from("f64"),
        min: f64::MIN,
        max: f64::MAX,
    }
    .print_range();

    // number literals
    println!("decimal: {}", 12_345_678);
    println!("   hexa: {}", 0x12ef);
    println!("   octa: {}", 0o777);
    println!(" binary: {}", 0b1010_0101);
    println!("   byte: {}", b'A');
}
