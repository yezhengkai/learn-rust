# The Rust Programming Language Notes
- [The Rust Programming Language Notes](#the-rust-programming-language-notes)
  - [Ch 3.1 Variables](#ch-31-variables)
  - [Ch 3.2 Data Types](#ch-32-data-types)
    - [Scalar Types](#scalar-types)
      - [Integer Types](#integer-types)
      - [Floating-Point Types](#floating-point-types)
      - [The Boolean Type](#the-boolean-type)
      - [The Character Type](#the-character-type)
    - [Compound Types](#compound-types)
      - [The Tuple Type](#the-tuple-type)
      - [The Array Type](#the-array-type)
  - [Ch 3.3 Functions](#ch-33-functions)
    - [Parameters](#parameters)
    - [Statements and Expressions](#statements-and-expressions)
    - [Functions with Return Values](#functions-with-return-values)
  - [Ch 3.4 Comments](#ch-34-comments)
  - [Ch 3.5 Control Flow](#ch-35-control-flow)
    - [`if` Expressions](#if-expressions)
    - [Repetition with Loops](#repetition-with-loops)
      - [Repeating Code with `loop`](#repeating-code-with-loop)
      - [Returning Values from Loops](#returning-values-from-loops)
      - [Loop Labels to Disambiguate Between Multiple Loops](#loop-labels-to-disambiguate-between-multiple-loops)
      - [Conditional Loops with `while`](#conditional-loops-with-while)
      - [Looping Through a Collection with `for`](#looping-through-a-collection-with-for)
  - [Ch 4.1 What is Ownership?](#ch-41-what-is-ownership)
    - [Ownership Rules](#ownership-rules)
    - [Variable Scope](#variable-scope)
    - [The String Type](#the-string-type)
    - [Memory and Allocation](#memory-and-allocation)
  - [Cargo](#cargo)

The `main` function is special: it is always the first code that runs in every executable Rust program.

Using `!` means you are calling a macro rather than a normal function, and macros don't always follow the same rules as functions.

A `prelude` is a collection of names that are automatically brought into scope of every module in a crate. [Preludes](https://doc.rust-lang.org/reference/names/preludes.html)

An *associated function* is a function that's implemented on a type.

The `&` indicates that this argument is a *reference*, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. Like variables, references are **immutable** by default.

`Result` is an *enumeration*, often called an *enum*, which is a type that can be in one of multiple possible states. We call each possible state a ***variant***.

Result's variants
- `Ok`: Operation was successful, and inside `Ok` is the successfully generated value.
- `Err`: Contains information about how or why the operation failed.

**Crate** is a collection of Rust source code files. There are two types of crates
- *Binary crate*, which is an executable.
- *Library crate*, which contains code that is intended to be used in other programs and can't be executed on its own.

A `match` expression is made up of ***arms***. An arm consists of a ***pattern*** to match against, and the code that should be run if the value given to match fits that arm's pattern. Rust takes the value given to `match` and looks through each arm's pattern in turn. Patterns and the `match` construct are powerful Rust features: they let you express a variety of situations your code might encounter and they make sure you handle them all. 

The colon (\:) after variable tells Rust we'll annotate the variable's type. For example, `let x: u32 = 1;`.

The `loop` keyword creates an infinite loop.

## Ch 3.1 Variables
Like immutable variables, ***constants*** are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.
1. Using `mut` with constants is not allowed.
2. Constants aren't just immutable by default—they're always immutable.
3. Constants are declared using the `const` keyword instead of the `let` keyword, and the type of the value *must* be annotated.
4. Constants can be declared in any scope, including the global scope.
5. Constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

***Shadowing*** lets us **reuse variable name** rather than forcing us to create two unique variables (Rustaceans say that the first variable is shadowed by the second). In effect, the second variable overshadows the first, taking any uses of the variable name to itself until either **it itself is shadowed or the scope ends**. There are some differences between *shadowing* and marking a variable as `mut` when reassigning variable:
|             | Shadowing(through `let`) | `mut` |
| ----------- | :----------------------: | :---: |
| chang type  |           true           | false |
| chang value |           true           | true  |

> Shadowing is often used when you want to **convert a value from one type to another type**.

## Ch 3.2 Data Types
Every value in Rust is of a certain ***data type***, which tells Rust what kind of data is being specified so it knows how to work with that data. There are two data type subsets: **scalar** and **compound**.
> Rust is a ***statically typed*** language, which means that it must know the types of all variables at compile time.

### Scalar Types
A *scalar* type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
#### Integer Types

Table: Integer Types in Rust
| Length  | Signed  | Unsigned |
| ------- | ------- | -------- |
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |
> ℹ️ Signed numbers are stored using [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement) representation.

Integer types default to `i32`. The primary situation in which you'd use `isize` or `usize` is when indexing some sort of collection.

Range of integer:
- Signed variant: $[-(2^{n - 1}), 2^{n - 1} - 1]$
  - `i8`: $[-(2^7), 2^7 - 1]$
- Unsigned variant: $[0, 2^n - 1]$
  - `u8`: $[0, 2^{8} - 1]$

The `isize` and `usize` types depend on the **architecture** of the computer your program is running on, which is denoted in the table as "arch": 64 bits if you're on a 64-bit architecture and 32 bits if you're on a 32-bit architecture.

Number literals that can be multiple numeric types allow a type *suffix*, such as `57u8`, to designate the type. Number literals can also use `_` as a visual separator to make the number easier to read, such as `1_000`.

Table: Integer Literals in Rust
| Number literals | Example     |
| --------------- | ----------- |
| Decimal         | `98_222`      |
| Hex             | `0xff`        |
| Octal           | `0o77`        |
| Binary          | `0b1111_0000` |
| Byte(`u8` only) | `b'A'`        |

#### Floating-Point Types
Rust's floating-point types are `f32` and `f64`, which are 32 bits and 64 bits in size, respectively. 
- The default type is `f64`.
- All floating-point types are signed.

#### The Boolean Type
- `true` and `false`
- Booleans are **one byte** in size
- keyword `bool`

#### The Character Type
- `char` type is the **most primitive alphabetic type**
- Specifying `char` literals with **single quotes** is different from specifying string literals with double quotes
- `char` type is **four bytes** in size and represents a **Unicode** Scalar Value
- Note that, a "character" isn't really a concept in Unicode, so your human intuition for what a "character" is may not match up with what a `char` is in Rust.

### Compound Types
*Compound types* can group multiple values into one type. Rust has two primitive compound types: `tuples` and `arrays`.

#### The Tuple Type
- Groups multiple values of **various types** together
- **Fixed length**: once declared, they cannot grow or shrink in size
- Create a tuple by writing a comma-separated list of values inside parentheses.
  - `let t = (500, 6.4, 1);`
  - With type annotation, `let t: (i32, f64, u8) = (500, 6.4, 1);`
- *Destructuring*: `let tup = (500, 6.4, 1); let (x, y, z) = tup;`
- Access a tuple element directly by using a period (`.`) followed by the index of the value we want to access. For example: `let first_elem = tup.0;`
- The tuple without any values has a special name, *unit*.
  - This value and its corresponding type are both written `()` and represent an empty value or an empty return type.
  - Expressions implicitly return the unit value if they don't return any other value.

#### The Array Type
- Unlike a tuple, every element of an array must have the **same type**.
- **Fixed length**: once declared, they cannot grow or shrink in size
  - data allocated on the stack rather than the heap
  - A *vector* is a similar collection type provided by the standard library that is allowed to grow or shrink in size.
- Create an array by writing a comma-separated list inside square brackets
  - `let a = [1, 2, 3, 4, 5];`
  - With annotation (`[type; #elem]`): `let a: [i32; 5] = [1, 2, 3, 4, 5];`
  - Initialization with same value: `let a = [3; 5];` == `let a = [3, 3, 3, 3, 3];`
- Access elements of an array using indexing, like this: `let first_elem = array[0];`

## Ch 3.3 Functions
- Define function: 
  ```rust
  fn fn_name(param_1: param_1_type, ...) -> return_type {
      ...
  }
  ```
- `main` function is the entry point of many programs
-  Function naming convention: `snake_case`

### Parameters
- *Parameters* are special variables that are part of a function's signature
- We can provide concrete values for parameters. Technically, the concrete values are called *arguments*
  >  In casual conversation, people tend to use the words parameter and argument interchangeably for either the variables in a function's definition or the concrete values passed in when you call a function.
- *Must* declare the type of each parameter

### Statements and Expressions
- Rust is an **expression-based language**
- **Statements** are instructions that perform some action and **do not return a value**.
  - `let y = 6;`
  - Function definitions
- **Expressions** evaluate to a resultant value.
  - Expressions can be part of statements: the `6` in the statement `let y = 6;`
  - Calling a function
  - Calling a macro
  - A new scope block created with curly brackets
    ``` rust
    let y = {
        let x = 3;
        x + 1 // doesn't have a semicolon at the end
    };
    /* This expression:
    {
        let x = 3;
        x + 1
    }
    */
    ```
  - **Expressions do not include ending semicolons.** If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. 

### Functions with Return Values
- We don't name return values, but we must **declare their type** after an arrow (`->`)
- The return value:
  - the value of the final expression in the block of the body of a function
  - *return early* from a function by using the `return` keyword and specifying a value, **but most functions return the last expression implicitly.**

## Ch 3.4 Comments
[Appendix B: Operators and Symbols](https://doc.rust-lang.org/book/appendix-02-operators.html) Table B-7
| Symbol  |  Explanation |
|---|---|
| `//`  | Line comment  |
| `//!`  | Inner line doc comment  |
| `///`  | Outer line doc comment  |
| `/*...*/`  | Block comment  |
| `/*!...*/`  | Inner block doc comment  |
| `/**...*/`  | Outer block doc comment  |

## Ch 3.5 Control Flow
### `if` Expressions
```rust
if condition {
  ...
}
```
```rust
if condition {
  ...
} else {
  ...
}
```
```rust
if condition {
  ...
} else if {
  ...
} else if {
  ...
} else {
  ...
}
```
- Blocks of code associated with the conditions in `if` expressions are sometimes called *arms*, just like the arms in `match` expressions
- condition *must* be a `bool`
- Rust will **not automatically try to convert non-Boolean types to a Boolean**. You must be **explicit** and always provide `if` with a Boolean as its condition
- Rust only executes the block for the first `true` condition, and once it finds one, it doesn't even check the rest
- If you use too many `else if` expressions, consider refactoring (maybe using `match`)
- Because `if` is an *expression*, we can use it on the right side of a `let` statement to assign the outcome to a variable
  - If the value types of the `if` and `else` arms are incompatible, a compilation error will occur.

### Repetition with Loops
#### Repeating Code with `loop`
- The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop
- Use `ctrl-c` to interrupt a program
- The `break` keyword in a loop tells the program when to stop executing the loop
- The `continue` keyword in a loop tells the program to skip over any remaining code in this iteration of the loop and go to the next iteration

#### Returning Values from Loops
To return a value from a `loop`, you can add the value to be returned after the `break` expression that stops the loop
```rust
let result = loop {
  ...
  break value;
};
```

#### Loop Labels to Disambiguate Between Multiple Loops
If you have loops within loops, `break` and `continue` apply to the innermost loop at that point. You can optionally specify a *loop label* on a loop that you can then use with `break` or `continue` to specify that those keywords apply to the labeled loop instead of the innermost loop.
Loop labels must begin with a single quote
```rust
'a: loop {...}
```

#### Conditional Loops with `while`
```rust
while condition {
  ...
}
```
 While the condition is `true`, the loop runs. When the condition ceases to be `true`, the program calls `break`, stopping the loop.

#### Looping Through a Collection with `for`
```rust
for element in collection {
  ...
}
```
- Using the `Range` provided by the standard library, it sequentially produces all numbers starting with one number and ending before the other.
  ```rust
  for number in (1..4).rev() {
      println!("{number}!");
  }
  // 3!
  // 2!
  // 1!
  ```

## Ch 4.1 What is Ownership?
💡 *Ownership* is a set of rules that govern how a Rust program manages memory.

All programs have to manage the way they use a computer's memory while running.
1. [Garbage collection (GC)](https://en.wikipedia.org/wiki/Garbage_collection_(computer_science))
   - A form of **automatic memory management**. The *garbage collector* attempts to reclaim memory that was allocated by the program, but is no longer referenced; such memory is called *garbage*.
   -  Garbage collection may take a significant proportion of a program's total processing time, and **affect performance** as a result.
   - **Resources other than memory**, such as network sockets, database handles, windows, file descriptors, and device descriptors, **are not typically handled by garbage collection, but rather by other methods (e.g. [destructors](https://en.wikipedia.org/wiki/Destructor_(computer_programming))).** Some such methods de-allocate memory also.
2. [Manual memory management](https://en.wikipedia.org/wiki/Manual_memory_management)
3. Ownership
   - Memory is managed through **a system of ownership with a set of rules that the *compiler* checks**.
   - If any of the rules are violated, the program won't compile.
   - Any features of ownership will not slow down the program while it's running.

Ownership addresses problems like tracking which parts of code use data on the heap, minimizing duplicate data on the heap, and cleaning up unused data on the heap to prevent running out of space.

> ℹ️ The Stack and the Heap
> 
> **Stack:**
> - Last In, First Out (LIFO)
> - Adding data is called *pushing onto the stack*
> - Removing data is called *popping off the stack*
> - All data stored on the stack must have a known, fixed size
> - When a function is called, passed arguments and local variables are pushed onto the stack and popped when the function ends.
> 
> **Heap:**
> - Less organized: when you put data on the heap, you request a certain amount of space.
> - *Allocating on the heap* (Abbreviated as *Allocating*): The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a **pointer**, which is the address of that location.
>    - Pointer (known, fixed size) can be stored on the stack
>    - To get actual data, we must follow the pointer
> - Data whose size is unknown at compile time or whose size may change must be stored on the heap
>
> Comparison:
> - Pushing to the stack is faster than allocating on the heap
> - Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there
> - Both the stack and the heap are parts of memory available to your code to use at **runtime**.

### Ownership Rules
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of **scope**, the value will be dropped.

### Variable Scope
A **scope** is the range within a program for which an item is valid.

The variable is valid from the point at which it's declared until the end of the current scope.
```rust
{                      // s is not valid here, it’s not yet declared
    // s refers to a string literal, where the value of the string is hardcoded into the text of our program.
    let s = "hello";   // s is valid from this point forward
    // do stuff with s
}                      // this scope is now over, and s is no longer valid
```
- When `s` comes into scope, it is valid.
- It remains valid until it goes out of scope.

### The String Type
*String literals*, where a string value is hardcoded into program. They aren’t suitable for every situation. Because
- immutable
- not every string value can be known when we write our code

For situations where string literals are not suitable, Rust has a second string type, `String`. This type **manages data allocated on the heap** and as such is able to store an amount of text that is unknown to us at compile time.

Create a `String` from a string literal using the from function, like so:
```rust
let s = String::from("hello");
```
This kind of string can be mutated:
```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{s}"); // This will print `hello, world!`
```

### Memory and Allocation



## Cargo
Use `cargo build` to compile a local package and all of its dependencies.
When your project is finally ready for **release**, you can use `cargo build --release` to compile it with **optimizations**. This command will create an executable in `target/release` instead of `target/debug`.

Use `cargo run` to compile the code and then run the resultant executable all in one command

Use `cargo check` to quickly check your code to make sure it compiles but doesn't produce an executable. (Check a local package and all of its dependencies for errors)

`cargo doc --open` command will build documentation provided by all your dependencies locally and open it in your browser.