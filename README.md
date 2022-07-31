# codec2.rs

Rust bindings for the [codec2](https://github.com/drowe67/codec2) library.

Provides the following packages:
  - [`libcodec2`](libcodec2): safe and ergonomic wrappers around the core libcodec2 function calls
  - [`codec2-sys`](codec2-sys): unsafe low-level bindings, automagically generated via [bindgen](https://github.com/rust-lang/rust-bindgen)

Additionally, this repo includes [`c2enc`](c2enc) which is a demo CLI program that imitates the original `c2enc`.

This project is unrelated to the [`codec2`](https://github.com/scriptjunkie/codec2) crate which is a partial pure-rust implementation.

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
