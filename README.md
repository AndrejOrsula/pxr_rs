# pxr_rs

<p align="left">
  <a href="https://crates.io/crates/pxr">                                        <img alt="crates.io" src="https://img.shields.io/crates/v/pxr_rs.svg"></a>
  <a href="https://github.com/AndrejOrsula/pxr_rs/actions/workflows/rust.yml">   <img alt="Rust"      src="https://github.com/AndrejOrsula/pxr_rs/actions/workflows/rust.yml/badge.svg"></a>
  <a href="https://github.com/AndrejOrsula/pxr_rs/actions/workflows/docker.yml"> <img alt="Docker"    src="https://github.com/AndrejOrsula/pxr_rs/actions/workflows/docker.yml/badge.svg"></a>
  <a href="https://codecov.io/gh/AndrejOrsula/pxr_rs">                           <img alt="codecov"   src="https://codecov.io/gh/AndrejOrsula/pxr_rs/branch/main/graph/badge.svg"></a>
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

The complete list of dependencies can be found within [`Dockerfile`](Dockerfile).

Enabling of additional non-default features might require additional dependencies. This is currently not tested/documented.

## Instructions

### <a href="#-rust"><img src="https://rustacean.net/assets/rustacean-flat-noshadow.svg" width="16" height="16"></a> Rust

Add `pxr` as a Rust dependency to your [`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html) manifest.

<!-- TODO[doc]: Update Cargo.toml dependency once the package can be reliably used from https://crates.io -->

```toml
[dependencies]
pxr = { git = "https://github.com/AndrejOrsula/pxr_rs.git" }
```

Note that the first build might take up to 50 minutes because the C++ library of OpenUSD will be automatically downloaded and compiled with the `vendored` feature enabled. The artifacts will be cached in `OUT_DIR` and reused for subsequent builds.

It is highly recommended to use `lld` or `mold` linker because `ld` might currently fail.

<details>
<summary><h3><a href="#-docker"><img src="https://www.svgrepo.com/show/448221/docker.svg" width="16" height="16"></a> Docker</h3></summary>

> To install [Docker](https://docs.docker.com/get-docker) on your system, you can run [`.docker/host/install_docker.bash`](.docker/host/install_docker.bash) to configure Docker with NVIDIA GPU support.
>
> ```bash
> .docker/host/install_docker.bash
> ```

#### Build Image

To build a new Docker image from [`Dockerfile`](Dockerfile), you can run [`.docker/build.bash`](.docker/build.bash) as shown below.

```bash
.docker/build.bash ${TAG:-latest} ${BUILD_ARGS}
```

#### Run Container

To run the Docker container, you can use [`.docker/run.bash`](.docker/run.bash) as shown below.

```bash
.docker/run.bash ${TAG:-latest} ${CMD}
```

#### Run Dev Container

To run the Docker container in a development mode (source code mounted as a volume), you can use [`.docker/dev.bash`](.docker/dev.bash) as shown below.

```bash
.docker/dev.bash ${TAG:-latest} ${CMD}
```

As an alternative, VS Code users familiar with [Dev Containers](https://code.visualstudio.com/docs/devcontainers/containers) can modify the included [`.devcontainer/devcontainer.json`](.devcontainer/devcontainer.json) to their needs. For convenience, [`.devcontainer/open.bash`](.devcontainer/open.bash) script is available to open this repository as a Dev Container in VS Code.

```bash
.devcontainer/open.bash
```

#### Join Container

To join a running Docker container from another terminal, you can use [`.docker/join.bash`](.docker/join.bash) as shown below.

```bash
.docker/join.bash ${CMD:-bash}
```

</details>

## License

This project is dual-licensed to be compatible with the Rust project, under either the [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) licenses.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
