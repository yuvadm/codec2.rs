# codec2.rs

Rust system bindings for the [codec2](https://github.com/drowe67/codec2) library.

Provides the following packages:
  - `libcodec2`: ergonomic high-level bindings
  - `codec2-sys`: low-level unsafe bindings, automagically generated via [bindgen](https://github.com/rust-lang/rust-bindgen).

Unrelated to the [`codec2`](https://github.com/scriptjunkie/codec2) crate which is a partial pure-rust implementation.

## Usage

Install with `cargo add`:

```bash
$ cargo add libcodec2
```

Or add manually to your `Cargo.toml`:

```
libcodec2 = "1.0.0"
```

## License

[LGPL-2.1](LICENSE)
