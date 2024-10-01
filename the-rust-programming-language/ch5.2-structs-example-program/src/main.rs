#[derive(Debug)] // outer attribute
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    main_naive();
    main_tuple();
    main_struct();
}

fn main_naive() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
}

fn main_tuple() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );

    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
}

fn main_struct() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
    println!("rect1 is {rect1:#?}");
    // Another way to print out a value using the Debug format is to use the dbg! macro, which takes ownership of an expression (as opposed to println!, which takes a reference), prints the file and line number of where that dbg! macro call occurs in your code along with the resultant value of that expression, and returns ownership of the value.

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1) // borrow the struct rather than take ownership of it
    );

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // dbg! returns ownership of the expression's value
        height: 50,
    };

    dbg!(&rect1); // We don't want dbg! to take ownership of rect1, so we use a reference to rect1
}

// note that accessing fields of a borrowed struct instance does not move the field values, which is why you often see borrows of structs
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
