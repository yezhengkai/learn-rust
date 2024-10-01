// https://stackoverflow.com/questions/68075910/how-to-disable-unused-variable-warning-in-rust
#![allow(unused)]

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Using Tuple Structs Without Named Fields to Create Different Types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Structs Without Any Fields
struct AlwaysEqual;

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // Using the Field Init Shorthand
    let user1 = build_user(
        String::from("someusername123"),
        String::from("someone@example.com"),
    );
    let user2 = build_user_field_init_shorthand(
        String::from("someusername123"),
        String::from("someone@example.com"),
    );

    // Creating Instances from Other Instances with Struct Update Syntax
    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    }; // user1.username moved
    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    }; // user2.username moved

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user_field_init_shorthand(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
