fn main() {
    // if ... else ...
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Rust will not automatically try to convert non-Boolean types to a Boolean.
    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }

    // if ... else if ... else
    // Rust only executes the block for the first true condition, and once it finds one, it doesn’t even check the rest
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let Statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}