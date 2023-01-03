#![allow(clippy::missing_safety_doc)]
use crate::{Metadata, Stream};
use std::ffi::CStr;
use std::os::raw::c_uint;

/// A trained Coqui STT model.
pub struct Model(pub(crate) *mut coqui_stt_sys::ModelState);

// these implementations are safe, as ModelState can be passed between threads safely
unsafe impl Send for Model {}
unsafe impl Sync for Model {}

impl Drop for Model {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: this is only called after the model has been disposed of
        unsafe { coqui_stt_sys::STT_FreeModel(self.0) }
    }
}

impl Model {
    /// Create a new model.
    ///
    /// # Errors
    /// Returns an error if the model path is invalid, or for other reasons.
    #[inline]
    pub fn new(model_path: impl Into<String>) -> crate::Result<Self> {
        Self::_new(model_path.into())
    }

    fn _new(model_path: String) -> crate::Result<Self> {
        let mut model_path = model_path.into_bytes();
        model_path.reserve_exact(1);
        model_path.push(b'\0');
        let model_path = CStr::from_bytes_with_nul(model_path.as_ref())?;

        let mut state = std::ptr::null_mut::<coqui_stt_sys::ModelState>();

        // SAFETY: creating a model is only done with a null pointer and a model path,
        // both of which have been checked
        let retval = unsafe {
            coqui_stt_sys::STT_CreateModel(model_path.as_ptr(), std::ptr::addr_of_mut!(state))
        };

        if let Some(e) = crate::Error::from_c_int(retval) {
            return Err(e);
        }

        if state.is_null() {
            return Err(crate::Error::Unknown);
        }

        Ok(Self(state))
    }

    /// Create a new model from a memory buffer.
    ///
    /// # Errors
    /// Returns an error if the model is invalid, or for other reasons.
    #[inline]
    #[cfg(not(target_os = "windows"))]
    pub fn new_from_buffer<'a>(buffer: impl AsRef<&'a [u8]>) -> crate::Result<Self> {
        Self::_new_from_buffer(buffer.as_ref())
    }

    #[inline]
    #[cfg(not(target_os = "windows"))]
    fn _new_from_buffer(buffer: &[u8]) -> crate::Result<Self> {
        let mut state = std::ptr::null_mut::<coqui_stt_sys::ModelState>();

        // SAFETY: creating a model is only done with a null pointer and a model buffer
        // both of which have been checked
        let retval = unsafe {
            coqui_stt_sys::STT_CreateModelFromBuffer(
                buffer.as_ptr().cast::<std::os::raw::c_char>(),
                buffer.len() as c_uint,
                std::ptr::addr_of_mut!(state),
            )
        };

        if let Some(e) = crate::Error::from_c_int(retval) {
            return Err(e);
        }

        if state.is_null() {
            return Err(crate::Error::Unknown);
        }

        Ok(Self(state))
    }

    /// Take this model, and return the inner model state.
    ///
    /// This is useful if the safe API does not provide something you need.
    ///
    /// # Safety
    /// Once this is called, the memory management of the model is no longer handled for you.
    ///
    /// You must not forget to call `STT_FreeModel` once you are done
    /// with the pointer to dispose of the model properly.
    #[inline]
    #[must_use]
    pub unsafe fn into_inner(self) -> *mut coqui_stt_sys::ModelState {
        let manual_drop = std::mem::ManuallyDrop::new(self);

        manual_drop.0
    }

    /// Create a new model from an existing model state.
    ///
    /// # Safety
    /// You must ensure `state` is a valid model state.
    #[inline]
    pub const unsafe fn from_model_state(state: *mut coqui_stt_sys::ModelState) -> Self {
        Self(state)
    }

    /// Enable an external scorer for this model.
    ///
    /// # Errors
    /// Returns an error if the `scorer_path`/file pointed to is invalid in some way.
    #[inline]
    pub fn enable_external_scorer(&mut self, scorer_path: impl Into<String>) -> crate::Result<()> {
        self._enable_external_scorer(scorer_path.into())
    }

    #[inline]
    fn _enable_external_scorer(&mut self, scorer_path: String) -> crate::Result<()> {
        let mut scorer_path = scorer_path.into_bytes();
        scorer_path.reserve_exact(1);
        scorer_path.push(b'\0');
        let scorer_path = CStr::from_bytes_with_nul(scorer_path.as_ref())?;
        handle_error!(coqui_stt_sys::STT_EnableExternalScorer(
            self.0,
            scorer_path.as_ptr()
        ))
    }

    /// Enable an external scorer for this model, loaded from a buffer in memory.
    ///
    /// # Errors
    /// Returns an error if the scorer in memory is invalid in some way.
    #[inline]
    #[cfg(not(target_os = "windows"))]
    pub fn enable_external_scorer_from_buffer(
        &mut self,
        buffer: impl AsRef<[u8]>,
    ) -> crate::Result<()> {
        self._enable_external_scorer_from_buffer(buffer.as_ref())
    }

    #[inline]
    #[cfg(not(target_os = "windows"))]
    fn _enable_external_scorer_from_buffer(&mut self, buffer: &[u8]) -> crate::Result<()> {
        handle_error!(coqui_stt_sys::STT_EnableExternalScorerFromBuffer(
            self.0,
            buffer.as_ptr().cast::<std::os::raw::c_char>(),
            buffer.len() as c_uint
        ))
    }

    /// Disable an external scorer that was previously set up with
    /// [`enable_external_scorer`](crate::Model::enable_external_scorer).
    ///
    /// # Errors
    /// Returns an error if an error happened while disabling the scorer.
    #[inline]
    pub fn disable_external_scorer(&mut self) -> crate::Result<()> {
        handle_error!(coqui_stt_sys::STT_DisableExternalScorer(self.0))
    }

    /// Add a hot-word and its boost.
    ///
    /// Words that don’t occur in the scorer (e.g. proper nouns),
    /// or strings that contain spaces won't be taken into account.
    ///
    /// # Errors
    /// Passes through any errors from the C library. See enum [`Error`](crate::Error).
    #[inline]
    pub fn add_hot_word(&mut self, word: impl Into<String>, boost: f32) -> crate::Result<()> {
        self._add_hot_word(word.into(), boost)
    }

    #[inline]
    fn _add_hot_word(&mut self, word: String, boost: f32) -> crate::Result<()> {
        let mut word = word.into_bytes();
        word.reserve_exact(1);
        word.push(b'\0');
        let word = CStr::from_bytes_with_nul(word.as_ref())?;
        handle_error!(coqui_stt_sys::STT_AddHotWord(self.0, word.as_ptr(), boost))
    }

    /// Remove entry for a hot-word from the hot-words map.
    ///
    /// # Errors
    /// Passes through any errors from the C library. See enum [`Error`](crate::Error).
    ///
    /// Additionally, if the input word contains a NUL character anywhere in it, returns an error.
    #[inline]
    pub fn erase_hot_word(&mut self, word: impl Into<String>) -> crate::Result<()> {
        self._erase_hot_word(word.into())
    }

    #[inline]
    fn _erase_hot_word(&mut self, word: String) -> crate::Result<()> {
        let mut word = word.into_bytes();
        word.reserve_exact(1);
        word.push(b'\0');
        let word = CStr::from_bytes_with_nul(word.as_ref())?;
        handle_error!(coqui_stt_sys::STT_EraseHotWord(self.0, word.as_ptr()))
    }

    /// Removes all elements from the hot-words map.
    ///
    /// # Errors
    /// Passes through any errors from the C library. See enum [`Error`](crate::Error).
    #[inline]
    pub fn clear_hot_words(&mut self) -> crate::Result<()> {
        handle_error!(coqui_stt_sys::STT_ClearHotWords(self.0))
    }

    /// Set hyperparameters alpha and beta of the external scorer.
    ///
    /// `alpha` is the alpha hyperparameter of the decoder. Language model weight.
    ///
    /// `beta` is the beta hyperparameter of the decoder. Word insertion weight.
    ///
    /// # Errors
    /// Passes through any errors from the C library. See enum [`Error`](crate::Error).
    #[inline]
    pub fn set_scorer_alpha_beta(&mut self, alpha: f32, beta: f32) -> crate::Result<()> {
        handle_error!(coqui_stt_sys::STT_SetScorerAlphaBeta(self.0, alpha, beta))
    }

    /// Return the sample rate expected by a model in Hz.
    #[inline]
    #[must_use]
    pub fn get_sample_rate(&self) -> i32 {
        unsafe { coqui_stt_sys::STT_GetModelSampleRate(self.0 as *const _) }
    }

    /// Use the Coqui STT model to convert speech to text.
    ///
    /// `buffer` should be a 16-bit, mono, raw audio signal
    /// at the appropriate sample rate, matching what the model was trained on.
    /// The required sample rate can be obtained from [`get_sample_rate`](crate::Model::get_sample_rate).
    ///
    /// # Errors
    /// Passes through any errors from the C library. See enum [`Error`](crate::Error).
    ///
    /// Additionally, if the returned string is not valid UTF-8, this function returns an error.
    #[allow(clippy::missing_inline_in_public_items)]
    pub fn speech_to_text(&mut self, buffer: &[i16]) -> crate::Result<String> {
        let ptr = unsafe {
            coqui_stt_sys::STT_SpeechToText(self.0, buffer.as_ptr(), buffer.len() as c_uint)
        };

        if ptr.is_null() {
            return Err(crate::Error::Unknown);
        }

        // SAFETY: STT_SpeechToText will always return a valid CStr
        let cstr = unsafe { CStr::from_ptr(ptr) };
        let mut unchecked_str = Vec::new();
        unchecked_str.extend_from_slice(cstr.to_bytes());

        // SAFETY: the pointer the string points to is not used anywhere after this call
        unsafe { coqui_stt_sys::STT_FreeString(ptr) }

        Ok(String::from_utf8(unchecked_str)?)
    }

    /// Use the Coqui STT model to convert speech to text and output results including metadata.
    ///
    /// `buffer` should be a 16-bit, mono, raw audio signal
    /// at the appropriate sample rate, matching what the model was trained on.
    /// The required sample rate can be obtained from [`get_sample_rate`](crate::Model::get_sample_rate).
    ///
    /// `num_results` is the maximum number of possible transcriptions to return.
    /// Note that it is not guaranteed this many will be returned at minimum,
    /// but there will never be more than this number at maximum.
    ///
    /// # Errors
    /// Passes through any errors from the C library. See enum [`Error`](crate::Error).
    #[inline]
    pub fn speech_to_text_with_metadata(
        &mut self,
        buffer: &[i16],
        num_results: u32,
    ) -> crate::Result<Metadata> {
        let ptr = unsafe {
            coqui_stt_sys::STT_SpeechToTextWithMetadata(
                self.0,
                buffer.as_ptr(),
                buffer.len() as c_uint,
                num_results,
            )
        };

        if ptr.is_null() {
            return Err(crate::Error::Unknown);
        }

        Ok(crate::Metadata::new(ptr))
    }

    /// Convert this model into one used for streaming inference states.
    ///
    /// Note that this requires exclusive access to the model,
    /// so it is not possible to use the same model for multiple streams concurrently.
    ///
    /// # Errors
    /// Passes through any errors from the C library. See enum [`Error`](crate::Error).
    #[allow(clippy::missing_inline_in_public_items)]
    pub fn as_streaming(&mut self) -> crate::Result<Stream> {
        let mut state = std::ptr::null_mut();

        let retval = unsafe { coqui_stt_sys::STT_CreateStream(self.0, &mut state) };

        if let Some(e) = crate::Error::from_c_int(retval) {
            return Err(e);
        }

        if state.is_null() {
            return Err(crate::Error::Unknown);
        }

        Ok(Stream {
            model: self,
            state,
            already_freed: false,
        })
    }
}
