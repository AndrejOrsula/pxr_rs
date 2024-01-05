# pxr_rs

<p align="left">
  <a href="https://crates.io/crates/pxr">                                        <img alt="crates.io" src="https://img.shields.io/crates/v/pxr_rs.svg"></a>
  <!-- <a href="https://github.com/AndrejOrsula/pxr_rs/actions/workflows/rust.yml">   <img alt="Rust"      src="https://github.com/AndrejOrsula/pxr_rs/actions/workflows/rust.yml/badge.svg"></a> -->
  <!-- <a href="https://github.com/AndrejOrsula/pxr_rs/actions/workflows/docker.yml"> <img alt="Docker"    src="https://github.com/AndrejOrsula/pxr_rs/actions/workflows/docker.yml/badge.svg"></a> -->
  <!-- <a href="https://codecov.io/gh/AndrejOrsula/pxr_rs">                           <img alt="codecov"   src="https://codecov.io/gh/AndrejOrsula/pxr_rs/branch/main/graph/badge.svg"></a> -->
</p>

Rust interface for [OpenUSD](https://openusd.org).

## Status

This project is in early development and is not ready for production use. Not all of the OpenUSD API is currently exposed.

Documentation and examples are currently lacking but will be the focus once the crates are more stable.

Currently, OpenUSD 22.11 compiled via build options specified via the [default features of pxr_sys](pxr_sys/Cargo.toml) is the primary target upon which the project is being developed.

## Overview

The workspace contains these packages:

- **[pxr_build](pxr_build):** Helper crate for building OpenUSD bindings
- **[pxr_sys](pxr_sys):** Unsafe Rust bindings for OpenUSD
- **[pxr](pxr_rs):** Safe Rust bindings for OpenUSD (WIP)

Most of the bindings are automatically generated from the OpenUSD headers using [autocxx](https://github.com/google/autocxx), while some are hand-written via [rust-cpp](https://github.com/mystor/rust-cpp) and additional macros inside [pxr_build](pxr_build). The C++ library of OpenUSD can be automatically downloaded and compiled during the cargo build process if the `vendored` feature is enabled.

## Dependencies

- `cmake`, `clang`, `python3-dev` for FFI bindings generation.
- `libarchive` as a transitive dependency of [`compress-tools`](https://github.com/OSSystems/compress-tools-rs).

Enabling of additional non-default features might require additional dependencies. This is currently not tested/documented.

## Instructions

### <a href="#-rust"><img src="https://rustacean.net/assets/rustacean-flat-noshadow.svg" width="16" height="16"></a> Rust

Add `pxr` as a Rust dependency to your [`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html) manifest.

<!-- TODO[doc]: Update Cargo.toml dependency once the package can be reliably used from https://crates.io -->

```toml
[dependencies]
pxr = { git = "https://github.com/AndrejOrsula/pxr_rs.git" }
```

## License

This project is dual-licensed to be compatible with the Rust project, under either the [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) licenses.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
