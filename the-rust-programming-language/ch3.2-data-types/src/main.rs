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
    // addition
    let _sum = 5 + 10;
    // subtraction
    let _difference = 95.5 - 4.3;
    // multiplication
    let _product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    println!("{}", quotient);
    let _truncated = -5 / 3; // Results in -1
    // remainder
    let remainder = 43 % 5; //
    println!("{}", remainder);
}
