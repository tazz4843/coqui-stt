#![deny(missing_docs)]
//! A safe wrapper around the [Coqui STT](https://stt.readthedocs.io/en/latest) API
//!
//! Typically, to use this,
//! * start by creating a [`Model`](Model),
//! * if you have a scorer to load, call [`enable_external_scorer`](Model::enable_external_scorer),
//! * then call [`speech_to_text`](Model::speech_to_text) to run the algorithm.
//!
//! # Features
//! No features are enabled by default.
//!
//! * `raw-bindings`: exposes the [`coqui-stt-sys`](coqui_stt_sys) crate at the root under the same name.

#[macro_use]
mod helpers;

mod candidate_transcript;
mod errors;
mod metadata;
mod model;
mod stream;
mod token_metadata;

pub use candidate_transcript::{CandidateTranscript, OwnedCandidateTranscript};
pub use errors::{Error, Result};
pub use metadata::{Metadata, OwnedMetadata};
pub use model::Model;
pub use stream::Stream;
pub use token_metadata::{OwnedTokenMetadata, TokenMetadata};

#[cfg(feature = "raw_bindings")]
pub use coqui_stt_sys;
