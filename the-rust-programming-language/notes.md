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
    - [Variables and Data Interacting with Move](#variables-and-data-interacting-with-move)
    - [Variables and Data Interacting with Clone](#variables-and-data-interacting-with-clone)
    - [Stack-Only Data: Copy](#stack-only-data-copy)
    - [Ownership and Functions](#ownership-and-functions)
    - [Return Values and Scope](#return-values-and-scope)
  - [Ch 4.2 References and Borrowing](#ch-42-references-and-borrowing)
    - [Mutable References](#mutable-references)
    - [Dangling References](#dangling-references)
    - [The Rules of References](#the-rules-of-references)
  - [Ch 4.3 The Slice Type](#ch-43-the-slice-type)
    - [String Slices](#string-slices)
    - [String Literals as Slices](#string-literals-as-slices)
    - [String Slices as Parameters](#string-slices-as-parameters)
    - [Other Slices](#other-slices)
  - [Ch 5.1 Defining and Instantiating Structs](#ch-51-defining-and-instantiating-structs)
    - [Using the Field Init Shorthand](#using-the-field-init-shorthand)
    - [Creating Instances from Other Instances with Struct Update Syntax](#creating-instances-from-other-instances-with-struct-update-syntax)
    - [Using Tuple Structs Without Named Fields to Create Different Types](#using-tuple-structs-without-named-fields-to-create-different-types)
    - [Unit-Like Structs Without Any Fields](#unit-like-structs-without-any-fields)
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
{                      // s is not valid here, it's not yet declared
    // s refers to a string literal, where the value of the string is hardcoded into the text of our program.
    let s = "hello";   // s is valid from this point forward
    // do stuff with s
}                      // this scope is now over, and s is no longer valid
```
- When `s` comes into scope, it is valid.
- It remains valid until it goes out of scope.

### The String Type
*String literals*, where a string value is hardcoded into program. They aren't suitable for every situation. Because
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

With the `String` type, in order to support a **mutable**, **growable** piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:
- The memory must be requested from the memory allocator at runtime.
  - When we call String::from, its implementation requests the memory it needs. 
- We need a way of returning this memory to the allocator when we're done with our `String`.
  - In languages with a *garbage collector (GC)*, the GC keeps track of and cleans up memory that isn't being used anymore, and we don't need to think about it.
  - In most languages without a GC, it's our responsibility to identify when memory is no longer being used and to call code to explicitly free it.
  - **The Rust way**: the memory is **automatically returned** once the variable that owns it goes **out of scope**.
  ```rust
  {
      let s = String::from("hello"); // s is valid from this point forward
      // do stuff with s
  }                                  // this scope is now over, and s is no
                                     // longer valid
  ```

When a variable goes out of scope, Rust calls a special function for us. This function is called [`drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop), and it's where the author of `String` can put the code to return the memory. Rust calls `drop` automatically at the closing curly bracket.

> 💡 In C++, this pattern of deallocating resources at the end of an item's lifetime is sometimes called ***Resource Acquisition Is Initialization (RAII)***. The `drop` function in Rust will be familiar to you if you've used RAII patterns.

### Variables and Data Interacting with Move
The `i32` version:
```rust
let x = 5;
let y = x;
```
The code above is doing: "bind the value `5` to `x`; then make a **copy** of the value in `x` and bind it to `y`." Integers are simple values with a **known, fixed size**, and these two `5` values are pushed onto the **stack**. For more information, go to [Stack-Only Data: Copy](#stack-only-data-copy)

`String` version:
```rust
let s1 = String::from("hello");
let s2 = s1;
```
A `String` is made up of three parts, shown on the left: a **pointer** to the memory that holds the contents of the string, a **length**, and a **capacity**. This group of data is stored on the stack. On the right is the memory on the heap that holds the contents.
![Representation in memory of a String holding the value "hello" bound to s1](https://doc.rust-lang.org/book/img/trpl04-01.svg)
The *length* is how much memory, in bytes, the contents of the `String` are **currently** using. The *capacity* is the **total amount of memory**, in bytes, that the `String` has received from the allocator.

When we assign `s1` to `s2`, the `String` data is copied, **meaning we copy the pointer, the length, and the capacity that are on the stack**. **We do not copy the data on the heap that the pointer refers to**. The data representation in memory looks like (Note that the figure below is not the actual representation in memory)
![Representation in memory of the variable s2 that has a copy of the pointer, length, and capacity of s1](https://doc.rust-lang.org/book/img/trpl04-02.svg)

Above figure shows both data pointers pointing to the same location. This is a problem: when `s2` and `s1` go out of scope, they will both try to free the same memory. (*double free error*)

To ensure memory safety, after the line `let s2 = s1;`, **Rust considers `s1` as no longer valid**. Therefore, Rust doesn't need to free anything when `s1` goes out of scope.

***move***: In this example, we would say that `s1` was moved into `s2`. So, what actually happens is shown in figure below
![alt text](https://doc.rust-lang.org/book/img/trpl04-04.svg)

With only `s2` valid, when it goes out of scope it alone will free the memory, and we're done.
In addition, there's a design choice that's implied by this: **Rust will never automatically create "deep" copies of your data**. Therefore, **any automatic copying can be assumed to be inexpensive in terms of runtime performance**.

### Variables and Data Interacting with Clone
If we *do* want to deeply copy the **heap** data of the `String`, not just the stack data, we can use a common method called `clone`.
```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {s1}, s2 = {s2}");
```
This works just fine and **explicitly** produces the behavior shown in figure below, where the heap data does get copied.
![Rust copied the heap data as well](https://doc.rust-lang.org/book/img/trpl04-03.svg)

When you see a call to `clone`, you know that some arbitrary code is being executed and **that code may be expensive**. It's a visual indicator that something different is going on.

### Stack-Only Data: Copy
This code using integers works and is valid:
```rust
let x = 5;
let y = x;

println!("x = {x}, y = {y}");
```
We don't have a call to `clone`, but `x` is still valid and wasn't moved into `y`.
The reason is that types that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. Calling `clone` wouldn't do anything different from the usual shallow copying, and we can leave it out.

We can place `Copy` trait on types stored on the stack.  If a type implements the `Copy` trait, **variables that use it do not move**, but rather are trivially copied, making them still valid after assignment to another variable.

⚠️ Rust won't let us annotate a type with `Copy` if the type, or any of its parts, has implemented the `Drop` trait.

As a general rule, any group of simple scalar values can implement `Copy`, and nothing that requires allocation or is some form of resource can implement `Copy`. Here are some of the types that implement `Copy`:
- All the integer types, such as `u32`.
- The Boolean type, `bool`, with values `true` and `false`.
- All the floating-point types, such as `f64`.
- The character type, `char`.
- Tuples, if they only contain types that also implement `Copy`. For example, (`i32`, `i32`) implements Copy, but (`i32`, `String`) does not.

### Ownership and Functions
Passing a variable to a function will move or copy, just as assignment does.
```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
```
If we tried to use `s` after the call to `takes_ownership`, Rust would throw a compile-time error. These static checks protect us from mistakes.

### Return Values and Scope
```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```
The ownership of a variable follows the same pattern every time: assigning a value to another variable **moves** it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless ownership of the data has been moved to another variable.

❓ What if we want to let a function use a value but not take ownership?
✅ Rust has a feature for using a value without transferring ownership, called *references*.

## Ch 4.2 References and Borrowing
A *reference* is like a pointer in that it's an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // &s1 syntax lets us create a reference that refers to the value of s1 but does not own it

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.
```
Note that we pass `&s1` into `calculate_length` and, in its definition, we take `&String` rather than `String`. These **ampersands represent references**, and they allow you to **refer to some value without taking ownership of it**. We call the action of creating a reference *borrowing*.

Just as variables are immutable by default, so are references. We're not allowed to modify something we have a reference to.

![ A diagram of &String s pointing at String s1](https://doc.rust-lang.org/book/img/trpl04-05.svg)

> 💡 Note: The opposite of referencing by using `&` is `dereferencing`, which is accomplished with the dereference operator, `*`. We'll see some uses of the dereference operator in Chapter 8 and discuss details of dereferencing in Chapter 15.

### Mutable References
```rust
fn main() {
    let mut s = String::from("hello"); // change s to be mut

    change(&mut s); // create a mutable reference with &mut s
}

fn change(some_string: &mut String) { // accept a mutable reference
    some_string.push_str(", world");
}
```

Mutable references have one big restriction: **if you have a mutable reference to a value, you can have no other references to that value**.

The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion. The benefit of having this restriction is that **Rust can prevent data races at compile time**. A *data race* is similar to a race condition and happens when these three behaviors occur:
- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There's no mechanism being used to synchronize access to the data.

As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not *simultaneous* ones:
```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```

Rust enforces a similar rule for combining mutable and immutable references.
```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);
```
We also **cannot have a mutable reference while we have an immutable one to the same value.**

**Users of an immutable reference don't expect the value to suddenly change out from under them!** However, **multiple immutable references are allowed** because no one who is just reading the data has the ability to affect anyone else's reading of the data.

Note that a reference's scope starts from where it is introduced and continues through the last time that reference is used. The scopes of the immutable references `r1` and `r2` end after the `println!` where they are last used, which is before the mutable reference `r3` is created.
```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{r1} and {r2}");
// variables r1 and r2 will not be used after this point

let r3 = &mut s; // no problem
println!("{r3}");
```

### Dangling References
> 💡 [Dangling pointer](https://en.wikipedia.org/wiki/Dangling_pointer)
> **Dangling pointers** and **wild pointers** in computer programming are pointers that do not point to a valid object of the appropriate type. These are special cases of memory safety violations. More generally, **dangling references** and **wild references** are references that do not resolve to a valid destination.

In Rust the compiler guarantees that **references will never be dangling references**: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.
```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {  // dangle returns a reference to a String
    let s = String::from("hello");  // s is a new String

    &s// we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```
Because `s` is created inside dangle, **when the code of `dangle` is finished, `s` will be deallocated**. But we tried to return a reference to it. That means this reference would be pointing to an invalid `String`. That's no good! Rust won't let us do this.

The solution here is to return the String directly:
```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```
This works without any problems. Ownership is moved out, and nothing is deallocated.

### The Rules of References
- At any given time, you can have *either* one mutable reference *or* any number of immutable references.
- References must always be valid.

## Ch 4.3 The Slice Type
*Slices* let you reference a **contiguous sequence of elements** in a collection rather than the whole collection.
**A slice is a kind of reference, so it does not have ownership.**

### String Slices
A string slice is a reference to part of a String, and it looks like this:
```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

We create slices using a range within brackets by specifying `[starting_index..ending_index]`, where `starting_index` is the first position in the slice and `ending_index` is one more than the last position in the slice. Internally, the slice data structure stores the **starting position** and the **length** of the slice, which corresponds to `ending_index` minus `starting_index`.
![String slice referring to part of a String](https://doc.rust-lang.org/book/img/trpl04-06.svg)

Some `..` range syntax:
```rust
let s = String::from("hello");
let len = s.len();

let slice = &s[0..2];
let slice = &s[..2];

let slice = &s[3..len];
let slice = &s[3..];

let slice = &s[0..len];
let slice = &s[..];
```

> 💡 Note: String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error. For the purposes of introducing string slices, we are assuming ASCII only in this section; a more thorough discussion of UTF-8 handling is in the "Storing UTF-8 Encoded Text with Strings" section of Chapter 8.

### String Literals as Slices
```rust
let s:&str = "Hello, world!";
```
The type of `s` here is `&str`: it's a slice pointing to that specific point of the binary. This is also why string literals are immutable; `&str` is an **immutable reference**.

### String Slices as Parameters
```rust
fn first_word(s: &String) -> &str {
```
Improving the `first_word` function by using a string slice for the type of the `s` parameter
```rust
fn first_word(s: &str) -> &str {
```
If we have a string slice, we can pass that directly. If we have a `String`, we can pass a slice of the `String` or a reference to the `String`. This flexibility takes advantage of deref coercions, a feature we will cover in the "Implicit Deref Coercions with Functions and Methods" section of Chapter 15.

### Other Slices
Consider array:
```rust
let a = [1, 2, 3, 4, 5];
let slice: &[i32] = &a[1..3];
assert_eq!(slice, &[2, 3]);
```

## Ch 5.1 Defining and Instantiating Structs
Structs are similar to tuples in that both hold multiple related values.
- Like tuples, the pieces of a struct can be different types.
- Unlike with tuples, in a struct you'll name each piece of data so it's clear what the values mean. You don't have to rely on the order of the data to specify or access the values of an instance.

To define a struct, we enter the keyword `struct` and name the entire struct.
- A struct's name should describe the significance of the pieces of data being grouped together.
- Then, inside curly brackets, we define the **names** and **types** of the pieces of data, which we call *fields*. 
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

Create an instance by stating the *name of the struct* and then add *curly brackets* containing *key: value pairs*, where the keys are the names of the fields and the values are the data we want to store in those fields. The order of the fields is not important.
To access this user's email address, we use `user1.email`. If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field. Note that **the entire instance must be mutable**; Rust doesn't allow us to mark only certain fields as mutable.
```rust
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        sign_in_count: 1,
        email: String::from("someone@example.com"),
    };

    user1.email = String::from("anotheremail@example.com");
}
```

As with any expression, we can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance.
```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
```

### Using the Field Init Shorthand
Because the **parameter names and the struct field names are exactly the same**, we can use the *field init shorthand* syntax to rewrite `build_user` so it behaves exactly the same but doesn't have the repetition of `username` and `email`. (write `email` rather than `email: email`)
```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

### Creating Instances from Other Instances with Struct Update Syntax
Create a new User instance in `user2` regularly, without the update syntax.
```rust
fn main() {
    // --snip--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```
Using *struct update syntax*. The syntax `..` specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
```rust
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```
The `..user1` must come last to specify that any remaining fields should get their values from the corresponding fields in `user1`, but we can choose to specify values for as many fields as we want in any order, regardless of the order of the fields in the struct's definition.

💡 Note that the struct update syntax uses `=` like an assignment; this is because it **moves the data**, just as we saw in the "Variables and Data Interacting with Move' section. In this example, we can no longer use `user1` as a whole after creating `user2` because the `String` in the `username` field of `user1` was moved into `user2`. If we had given `user2` new `String` values for both `email` and `username`, and thus only used the `active` and `sign_in_count` values from `user1`, then `user1` would still be valid after creating `user2`. Both `active` and `sign_in_count` are types that implement the `Copy` trait, so the behavior we discussed in the "Stack-Only Data: Copy" section would apply.

### Using Tuple Structs Without Named Fields to Create Different Types
**Tuple structs** have the added meaning the struct name provides but don't have names associated with their fields; rather, they just have the types of the fields.
Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0); // Color type
    let origin = Point(0, 0, 0); // Point type
}
```
Tuple struct instances are similar to tuples in that you can destructure them into their individual pieces, and you can use a `.` followed by the index to access an individual value.

### Unit-Like Structs Without Any Fields
**Structs that don't have any fields!** These are called *unit-like structs* because they behave similarly to `()`, the unit type that we mentioned in "The Tuple Type" section.
**Unit-like structs can be useful when you need to implement a trait on some type but don't have any data that you want to store in the type itself.**
```rust
struct AlwaysEqual; // unit-like struct

fn main() {
    let subject = AlwaysEqual;
}
```

> 💡 Ownership of Struct Data
> In the `User` struct definition, we used the owned `String` type rather than the `&str` string slice type. This is a deliberate choice because **we want each instance of this struct to own all of its data** and for that **data to be valid for as long as the entire struct is valid**.
>
> It's also possible for structs to store references to data owned by something else, but to do so requires the use of ***lifetimes***, a Rust feature that we'll discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is. Let's say you try to store a reference in a struct without specifying lifetimes, like the following; **this won't work**:
> ```rust
> struct User {
>     active: bool,
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
> }
> 
> fn main() {
>     let user1 = User {
>         active: true,
>         username: "someusername123",
>         email: "someone@example.com",
>         sign_in_count: 1,
>     };
> }
> ```
> **The compiler will complain that it needs lifetime specifiers**

## Cargo
Use `cargo build` to compile a local package and all of its dependencies.
When your project is finally ready for **release**, you can use `cargo build --release` to compile it with **optimizations**. This command will create an executable in `target/release` instead of `target/debug`.

Use `cargo run` to compile the code and then run the resultant executable all in one command

Use `cargo check` to quickly check your code to make sure it compiles but doesn't produce an executable. (Check a local package and all of its dependencies for errors)

`cargo doc --open` command will build documentation provided by all your dependencies locally and open it in your browser.