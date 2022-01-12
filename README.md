This repository shows how to integrate generated code whose top-level file contains inner attributes (`#![...]`).

`include!(...);` cannot be used when the file to include has inner attributes ([bug](https://github.com/rust-lang/rust/issues/47995)). `#[path = "..."] mod foo;` works in that case, as long as the path can be written down, which excludes it depending on an environment variable like `OUT_DIR` which is where Rust scripts are meant to write generated code.

This crate shows a mix of both ways that does the trick. The code is generated in `$OUT_DIR/foo` and a helper file `$OUT_DIR/wrap.rs` is also written with the following contents:
```rust
pub mod foo;
```

The code in `src` can just do `include!(concat!(env!("OUT_DIR"), "/wrap.rs"));` and use module `foo`.
