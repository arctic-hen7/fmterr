<h1 align="center">fmterr</h1>

[![API Docs](https://img.shields.io/docsrs/fmterr?label=API%20Docs&style=for-the-badge)](https://docs.rs/fmterr)
[![Crate Page](https://img.shields.io/crates/v/fmterr?style=for-the-badge)](https://crates.io/crates/fmterr)
[![Top Language](https://img.shields.io/github/languages/top/arctic-hen7/fmterr?style=for-the-badge)]()

`fmterr` (pronounced _fumpterr_ obviously!) is an error handling and reporting tool for Rust that extends on `thiserror` to provide a very simple API for creating and formatting errors in Rust. If you have an error with a chain of sources and you want to display it to the user without bringing in something like `anyhow`, `fmterr` is for you! Just run `fmt_err(&err)` and you'll get a string error message that displays the source chain nicely for end users.

`fmterr` also supports setting custom properties on error `enum`s, like making every error variant have its own HTTP status code if you're building a server app.

## Usage

Note that `fmterr` supports two main feature flags: `fmt` and `def`, which are both enabled by default. You can use `fmt` if you only want to format errors with this module, or `def` if you only want to define errors with this module.

### Formatting Errors

You can use `fmterr` to format any error that implements `std::error::Error` (basically any error worth its salt) like so:

```rust
use fmterr::fmt_err;

// Imagine this is a super-complex error with a nested source
let err = std::io::Error::from(std::io::ErrorKind::NotFound);

let err_str = fmt_err(&err);
println!("{}", err_str);
```

That'll produce the following if you use a complex error:

```
Error: first error message

Caused by:
    second error message
    Caused by:
        third error message
```

Or just this if you use something without a source (with an `io::ErrorKind::NotFound`):

```
Error: entity not found
```

### Defining Errors

TODO

## Contributing

We appreciate all kinds of contributions, check out our [contributing guidelines](./CONTRIBUTING.md) for more information! Also, please be sure to follow our [code of conduct](./CODE_OF_CONDUCT.md).

## License

See [`LICENSE`](./LICENSE).
