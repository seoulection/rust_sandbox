# Rust Sandbox

A place for me to learn and practice Rust

## How to Run

Rust is cool in the sense that you can compile a `.rs` file, which will then create an executable that you can pass along to anyone. They don't need to have your code!

`cd` into any of the example project directories and run `./main`

## Useful Commands

`cargo new <project_name>` - creates a Rust project using Cargo

`cargo build` - compiles the project and creates an executable in `target/debug/<project_name>`

`cargo run` - same as `cargo build` but it runs the executable as well

`cargo check` - checks your code to see if it compiles, but doesn't produce an executable

`cargo doc --open` - builds documentation provided by all your dependencies locally and opens it in the browser
