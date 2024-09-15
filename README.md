# Learn Rust
Projects that helped me learn the Rust programming language.

## Notes
The `main` function is special: it is always the first code that runs in every executable Rust program.

Using `!` means you are calling a macro rather than a normal function, and macros don't always follow the same rules as functions.

A `prelude` is a collection of names that are automatically brought into scope of every module in a crate. [Preludes](https://doc.rust-lang.org/reference/names/preludes.html)

An *associated function* is a function that’s implemented on a type.

The `&` indicates that this argument is a *reference*, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. Like variables, references are **immutable** by default.

`Result` is an *enumeration*, often called an *enum*, which is a type that can be in one of multiple possible states. We call each possible state a ***variant***.

Result’s variants
- `Ok`: Operation was successful, and inside `Ok` is the successfully generated value.
- `Err`: Contains information about how or why the operation failed.

**Crate** is a collection of Rust source code files. There are two types of crates
- *Binary crate*, which is an executable.
- *Library crate*, which contains code that is intended to be used in other programs and can’t be executed on its own.

A `match` expression is made up of ***arms***. An arm consists of a ***pattern*** to match against, and the code that should be run if the value given to match fits that arm’s pattern. Rust takes the value given to `match` and looks through each arm’s pattern in turn. Patterns and the `match` construct are powerful Rust features: they let you express a variety of situations your code might encounter and they make sure you handle them all. 

The colon (\:) after variable tells Rust we’ll annotate the variable’s type. For example, `let x: u32 = 1;`.

The `loop` keyword creates an infinite loop.

Like immutable variables, ***constants*** are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.
1. Using `mut` with constants is not allowed.
2. Constants aren’t just immutable by default—they’re always immutable.
3. Constants are declared using the `const` keyword instead of the `let` keyword, and the type of the value *must* be annotated.
4. Constants can be declared in any scope, including the global scope.
5. Constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

***Shadowing*** lets us **reuse variable name** rather than forcing us to create two unique variables (Rustaceans say that the first variable is shadowed by the second). In effect, the second variable overshadows the first, taking any uses of the variable name to itself until either **it itself is shadowed or the scope ends**. There are some differences between *shadowing* and marking a variable as `mut` when reassigning variable:
|             | Shadowing(through `let`) | `mut` |
| ----------- | :----------------------: | :---: |
| chang type  |           true           | false |
| chang value |           true           | true  |

| Shadowing is often used when you want to **convert a value from one type to another type**.


### Cargo
`cargo doc --open` command will build documentation provided by all your dependencies locally and open it in your browser.

## References
- [The Rust Programming Language](https://doc.rust-lang.org/stable/book/title-page.html)