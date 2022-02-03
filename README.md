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

## MSRV

The MSRV is always the latest stable version,
currently 1.58.1 (2022-01-20) as of this writing.
