// https://stackoverflow.com/questions/68075910/how-to-disable-unused-variable-warning-in-rust
#![allow(unused)]

fn main() {
    // Integer
    let integer = 5u8; // == let integer: u8 = 5
    println!("{}", integer);
    let integer = 0x0090i16; // == let integer: i16 = 144
    println!("{}", integer);
    let integer = 1_234; // == let integer: i32 = 1234
    println!("{}", integer);

    // Floating-point
    let floating = 2.0; // f64
    println!("{}", floating);
    let floating = 2.0f32; // f32
    println!("{}", floating);
    let floating: f32 = 3.0; // f32
    println!("{}", floating);

    // Numeric Operations
    let sum = 5 + 10; // addition
    let difference = 95.5 - 4.3; // subtraction
    let product = 4 * 30; // multiplication
    let quotient = 56.7 / 32.2; // division
    println!("{}", quotient);
    let truncated = -5 / 3; // division, results in -1
    let remainder = 43 % 5; // remainder, results in 3
    println!("{}", remainder);

    // Boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    // Char
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; // use pattern matching to destructure a tuple value
    println!("The value of y is: {y}");

    // Array
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];  // let a = [3, 3, 3, 3, 3];
    let first = a[0];
    let second = a[1];
}
