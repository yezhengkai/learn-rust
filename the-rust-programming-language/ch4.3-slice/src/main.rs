fn main() {
    let mut s = String::from("hello world");

    // Return usize
    let _word = first_word_not_ideal(&s); // word will get the value 5
    
    // s.clear(); // this empties the String, making it equal to ""
    
    // _word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. _word is now totally invalid!

    // Return string slice &str
    let word = first_word(&s);

    // s.clear(); // error!
    /*
    Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a mutable reference. Because `clear` needs to truncate the `String`, it needs to get a mutable reference. The `println!` after the call to `clear` uses the reference in `word`, so the immutable reference must still be active at that point. Rust disallows the mutable reference in `clear` and the immutable reference in `word` from existing at the same time, and compilation fails. Not only has Rust made our API easier to use, but it has also eliminated an entire class of errors at compile time!
    */

    println!("the first word is: {word}");

    let my_string = String::from("hello world");

    // `first_word_improve` works on slices of `String`s, whether partial or whole
    let word = first_word_improve(&my_string[0..6]);
    let word = first_word_improve(&my_string[..]);
    // `first_word_improve` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word_improve(&my_string);

    let my_string_literal = "hello world";

    // `first_word_improve` works on slices of string literals, whether partial or whole
    let word = first_word_improve(&my_string_literal[0..6]);
    let word = first_word_improve(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_improve(my_string_literal);
}


fn first_word_not_ideal(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Improving the first_word function by using a string slice for the type of the s parameter
fn first_word_improve(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
