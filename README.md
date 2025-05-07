## Rust proc macro with shared context

![](./demo.gif)

Example of how Rust proc macros could have a shared context, so every macro crate is using the same metadata no matter in which order macros are expanded.

### Structure

- [ctx](./ctx) - Defines common context, logic for generating metadata from source code and functions to store / load metadata
- [example](./example/) - Binary that uses our macros
- [macros](./macros/) - Example of macros that use shared metadata

### How it works

The `ctx` crate analyzes the code of dependant crate `example` during execution of its [build.rs](./example/build.rs) and generates metadata into build directory. The `macros` crate uses `ctx` functionality to load metadata for the `example` crate during proc macro expanding stage and generates some code using loaded information.

## How to use it

- Execute `cargo run` to run it.
- Execute `cargo expand -p example` to see how generated code.
