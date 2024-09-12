# Learn Rust
Projects that helped me learn the Rust programming language.

## Notes
The `main` function is special: it is always the first code that runs in every executable Rust program.

Using `!` means you are calling a macro rather than a normal function, and macros don't always follow the same rules as functions.

A `prelude` is a collection of names that are automatically brought into scope of every module in a crate. [Preludes](https://doc.rust-lang.org/reference/names/preludes.html)

An *associated function* is a function that’s implemented on a type.

The `&` indicates that this argument is a *reference*, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. Like variables, references are **immutable** by default.

`Result` is an *enumeration*, often called an *enum*, which is a type that can be in one of multiple possible states. We call each possible state a *variant*. 
Result’s variants
- `Ok`: Operation was successful, and inside `Ok` is the successfully generated value.
- `Err`: Contains information about how or why the operation failed.
## References
- [The Rust Programming Language](https://doc.rust-lang.org/stable/book/title-page.html)