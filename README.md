# coqui-stt

Docs: https://docs.rs/coqui-stt/latest/coqui_stt

crates.io: https://crates.io/crates/coqui-stt

Github: https://github.com/tazz4843/coqui-stt

A simple, yet feature-filled wrapper around the
[coqui-stt](https://stt.readthedocs.io/en/latest) C API.

Handles all low-level things for you.
All you need to worry about is passing in a valid model, optionally scorer, and audio.

If you'd like to, audio streaming is supported with the `Stream` class.

You can gain extended metadata about an audio result with the `Metadata` class.

Some hidden functions are also exposed in the Rust API with `#[doc(hidden)]`.

## On Windows

### Compiling your code

The Coqui-STT C libraries need to be discoverable by the rust linker. For that, you can do either of the following:

-   Move them to a folder in your PATH variable.
-   Create a [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html) and provide cargo with the path to the libraries with `cargo:rustc-link-search` or `cargo:rustc-link-lib`

### Running your code

The libraries also have to be discoverable by the executable. If you followed the first option in the previous section, it will run with no extra effort; otherwise, you will need to copy the libraries to your current working directory (`target/<profile name>` by default). It is recommended that you use a tool such as [cargo-make](https://sagiegurari.github.io/cargo-make/) to automate this.

## MSRV

The MSRV is always the latest stable version,
currently 1.58.1 (2022-01-20) as of this writing.
