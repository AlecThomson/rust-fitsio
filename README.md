# rust-fitsio

FFI wrapper around cfitsio in Rust


[![Join the chat at https://gitter.im/mindriot101/rust-fitsio](https://badges.gitter.im/mindriot101/rust-fitsio.svg)](https://gitter.im/mindriot101/rust-fitsio?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![Build Status](https://travis-ci.org/mindriot101/rust-fitsio.svg?branch=master)](https://travis-ci.org/mindriot101/rust-fitsio)

## Installation

For the time being, it's best to stick to the development version from github.
The code is tested before being pushed and is relatively stable. Add this to
your `Cargo.toml` file:

```toml
[dependencies]
fitsio = { git = "https://github.com/mindriot101/rust-fitsio" }
```

If you want the latest release from `crates.io` then add the following:

```toml
[dependencies]
fitsio = "*"
```

Or pin a specific version:

```toml
[dependencies]
fitsio = "0.2.0"
```

This repository contains `fitsio-sys-bindgen` which generates the C wrapper using `bindgen` at build time. This requires clang to build, and as this is likely to not be available in general, I do not recommend using it. It is contained here but is not actively developed, and untested. Use at your own peril.

## Documentation

`fitsio` [![`fitsio` documentation](https://docs.rs/fitsio/badge.svg)](https://docs.rs/fitsio/)<br />
`fitsio-sys` [![`fitsio-sys` documentation](https://docs.rs/fitsio-sys/badge.svg)](https://docs.rs/fitsio-sys)<br />
`fitsio-sys-bindgen` [![`fitsio-sys-bindgen` documentation](https://docs.rs/fitsio-sys-bindgen/badge.svg)](https://docs.rs/fitsio-sys-bindgen)<br />

## Examples

Open a fits file

```rust
let f = fitsio::FitsFile::open("test.fits");
```
