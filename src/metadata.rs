use crate::{CandidateTranscript, OwnedCandidateTranscript};

/// An array of [`CandidateTranscript`](CandidateTranscript) objects computed by the model.
#[repr(transparent)]
pub struct Metadata(*mut coqui_stt_sys::Metadata);

unsafe impl Send for Metadata {}
unsafe impl Sync for Metadata {}

impl Drop for Metadata {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: this function is called with a pointer to self
        // at the end of this object's lifetime
        unsafe { coqui_stt_sys::STT_FreeMetadata(self.0) }
    }
}

impl Metadata {
    pub(crate) fn new(ptr: *mut coqui_stt_sys::Metadata) -> Self {
        if ptr.is_null() {
            unreachable!("attempted to construct Metadata with a null pointer");
        }
        Self(ptr)
    }

    /// Return an array of possible transcriptions.
    #[inline]
    #[must_use]
    pub fn transcripts(&self) -> &[CandidateTranscript] {
        // SAFETY: this object will never be constructed with a null pointer
        let data = unsafe { (*self.0).transcripts.cast() };
        let len = self.num_transcripts() as usize;

        // SAFETY: the inner objects will always be of type TokenMetadata,
        // and the length will always be proper
        unsafe { std::slice::from_raw_parts(data, len) }
    }

    /// Size of the transcripts array
    #[inline]
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn num_transcripts(&self) -> u32 {
        // SAFETY: this object will never be constructed with a null pointer
        unsafe { (*self.0).num_transcripts }
    }

    /// Convert this into an [`OwnedMetadata`](OwnedMetadata) struct.
    ///
    /// **Warning**: this can be an extremely expensive operation depending on
    /// how many transcriptions were returned, as well as the average length.
    #[inline]
    #[must_use]
    pub fn to_owned(&self) -> OwnedMetadata {
        OwnedMetadata(
            self.transcripts()
                .iter()
                .map(CandidateTranscript::to_owned)
                .collect(),
        )
    }
}

/// An owned variant of [`Metadata`](Metadata).
pub struct OwnedMetadata(Vec<OwnedCandidateTranscript>);

impl OwnedMetadata {
    /// Return an array of possible transcriptions.
    #[inline]
    #[must_use]
    pub fn transcripts(&self) -> &[OwnedCandidateTranscript] {
        self.0.as_slice()
    }

    /// Size of the transcripts array
    #[inline]
    #[must_use]
    pub fn num_transcripts(&self) -> u32 {
        self.0.len() as u32
    }

    /// Return the inner
    /// `Vec<`[`OwnedCandidateTranscript`](OwnedCandidateTranscript)`>`
    /// this data owns.
    #[inline]
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn into_inner(self) -> Vec<OwnedCandidateTranscript> {
        self.0
    }
}
