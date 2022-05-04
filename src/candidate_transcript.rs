use crate::{OwnedTokenMetadata, TokenMetadata};
use std::fmt::{Display, Formatter, Write};

/// A single transcript computed by the model,
/// including a confidence value and the metadata for its constituent tokens.
#[repr(transparent)]
pub struct CandidateTranscript(coqui_stt_sys::CandidateTranscript);

unsafe impl Send for CandidateTranscript {}
unsafe impl Sync for CandidateTranscript {}

impl CandidateTranscript {
    /// Return an array of tokens in this transcript.
    #[inline]
    #[must_use]
    pub fn tokens(&self) -> &[TokenMetadata] {
        let data = self.0.tokens.cast();
        let len = self.num_tokens() as usize;

        // SAFETY: the inner objects will always be of type TokenMetadata,
        // and the length will always be proper
        unsafe { std::slice::from_raw_parts(data, len) }
    }

    /// Approximated confidence value for this transcript.
    /// This is roughly the sum of the acoustic model logit values for
    /// each timestep/character that contributed to the creation of this transcript.
    #[inline]
    #[must_use]
    pub const fn confidence(&self) -> f64 {
        self.0.confidence
    }

    /// Total number of tokens in this transcript.
    #[inline]
    #[must_use]
    pub const fn num_tokens(&self) -> u32 {
        self.0.num_tokens
    }

    /// Convert this into an [`OwnedCandidateTranscript`](OwnedCandidateTranscript) struct.
    ///
    /// **Warning**: this can be very expensive depending on the total number of tokens in this object.
    #[inline]
    #[must_use]
    pub fn to_owned(&self) -> OwnedCandidateTranscript {
        let tokens = self.tokens().iter().map(|t| t.to_owned()).collect();
        OwnedCandidateTranscript {
            tokens,
            confidence: self.confidence(),
        }
    }
}

/// An owned variant of [`CandidateTranscript`](CandidateTranscript).
#[derive(Clone, Debug)]
pub struct OwnedCandidateTranscript {
    tokens: Vec<OwnedTokenMetadata>,
    confidence: f64,
}

impl OwnedCandidateTranscript {
    /// Return an array of tokens in this transcript.
    #[inline]
    #[must_use]
    pub fn tokens(&self) -> &[OwnedTokenMetadata] {
        &self.tokens[..]
    }

    /// Approximated confidence value for this transcript.
    /// This is roughly the sum of the acoustic model logit values for
    /// each timestep/character that contributed to the creation of this transcript.
    #[inline]
    #[must_use]
    pub const fn confidence(&self) -> f64 {
        self.confidence
    }

    /// Total number of tokens in this transcript.
    #[inline]
    #[must_use]
    pub fn num_tokens(&self) -> usize {
        self.tokens.len()
    }
}

impl Display for OwnedCandidateTranscript {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for token in &self.tokens {
            f.write_str(&token.text)?;
            f.write_char(' ')?;
        }
        Ok(())
    }
}
