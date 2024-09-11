Compile a local package and all of its dependencies
```shell
cargo build
```

Run executable
```shell
./target/debug/hello_cargo
```

Use `cargo run` to compile the code and then run the resultant executable all in one command:
```shell
cargo run
```

Quickly checks your code to make sure it compiles but doesn’t produce an executable. (Check a local package and all of its dependencies for errors)
```shell
cargo check
```

When your project is finally ready for **release**, you can use `cargo build --release` to compile it with **optimizations**. This command will create an executable in `target/release` instead of `target/debug`.