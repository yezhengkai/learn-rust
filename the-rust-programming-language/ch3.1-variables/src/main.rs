const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // Requires type annotations and only accepts a limited set of operations at compile time

fn main() {
    println!("The constant of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    let x = 5;
    println!("The value of x is: {x}");

    let mut x = x + 1; // shadowing

    {
        let x = x * 2; // shadowing
        println!("The value of x in the inner scope is: {x}");
    }

    x = 6;  // mutable
    println!("The value of x is: {x}");
}
