use crate::{Metadata, Model};
use std::ffi::CStr;
use std::mem::ManuallyDrop;
use std::sync::Arc;

/// Streaming inference state.
pub struct Stream {
    pub(crate) model: Arc<Model>,
    pub(crate) state: *mut coqui_stt_sys::StreamingState,
}

impl Drop for Stream {
    #[inline]
    fn drop(&mut self) {
        unsafe { coqui_stt_sys::STT_FreeStream(self.state) }
    }
}

impl Stream {
    /// Attempt to create a new `Stream`
    /// by cloning the [`Model`](Model) this one points to.
    ///
    /// # Errors
    /// Passes through any errors from the C library. See enum [`Error`](crate::Error).
    #[inline]
    pub fn try_clone(&self) -> crate::Result<Self> {
        Self::from_model(Arc::clone(&self.model))
    }

    /// Create a new `Stream` from a [`Model`](Model).
    ///
    /// # Errors
    /// Passes through any errors from the C library. See enum [`Error`](crate::Error).
    #[allow(clippy::missing_inline_in_public_items)]
    pub fn from_model(model: Arc<Model>) -> crate::Result<Self> {
        let mut state = std::ptr::null_mut::<coqui_stt_sys::StreamingState>();

        let retval = unsafe { coqui_stt_sys::STT_CreateStream(model.0, &mut state as *mut _) };

        if let Some(e) = crate::Error::from_c_int(retval) {
            return Err(e);
        }

        if state.is_null() {
            return Err(crate::Error::Unknown);
        }

        Ok(Self { model, state })
    }

    /// Get the inner pointer to the [`StreamingState`](coqui_stt_sys::StreamingState)
    /// of this `Stream`.
    ///
    /// # Safety
    /// Once this is called, the memory management of the `Stream` is no longer handled for you.
    ///
    /// The [`Model`] object must not be disposed of until all `Stream`s pointing to it are disposed of.
    ///
    /// Once you are done with the pointer, to dispose of the state properly,
    /// you must not forget to either (NOT BOTH):
    /// * call [`STT_FreeStream`],
    /// * recreate this object with [`from_ptr`]
    ///
    /// [`Model`]: Model
    /// [`STT_FreeStream`]: coqui_stt_sys::STT_FreeStream
    /// [`from_ptr`]: Stream::from_ptr
    #[inline]
    #[must_use]
    pub unsafe fn into_state(self) -> (Arc<Model>, *mut coqui_stt_sys::StreamingState) {
        let this = ManuallyDrop::new(self);
        (Arc::clone(&this.model), this.state)
    }

    /// Recreate a `Stream` with a pointer to a [`StreamingState`]
    /// and a pointer to the model the [`StreamingState`] references.
    ///
    /// # Safety
    /// * The `state` must point to a valid [`StreamingState`].
    /// * The `model` must point to the exact same [`Model`] the original `state` was created with.
    ///
    /// [`StreamingState`]: coqui_stt_sys::StreamingState
    /// [`Model`]: Model
    #[inline]
    pub unsafe fn from_ptr(model: Arc<Model>, state: *mut coqui_stt_sys::StreamingState) -> Self {
        Self { model, state }
    }

    /// Return a reference to the [`Model`](crate::Model) this wraps.
    #[inline]
    #[must_use]
    pub fn model(&self) -> Arc<Model> {
        Arc::clone(&self.model)
    }

    /// Feed audio samples to an ongoing streaming inference.
    #[inline]
    pub fn feed_audio(&mut self, buffer: &[i16]) {
        unsafe {
            coqui_stt_sys::STT_FeedAudioContent(
                self.state,
                buffer.as_ptr(),
                buffer.len() as std::os::raw::c_uint,
            );
        }
    }

    /// Compute the intermediate decoding of an ongoing streaming inference.
    ///
    /// # Errors
    /// Passes through any errors from the C library. See enum [`Error`](crate::Error).
    #[allow(clippy::missing_inline_in_public_items)]
    pub fn intermediate_decode(&self) -> crate::Result<String> {
        let ptr = unsafe { coqui_stt_sys::STT_IntermediateDecode(self.state as *const _) };

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

    /// Compute the intermediate decoding of an ongoing streaming inference,
    /// return results including metadata.
    ///
    /// # Errors
    /// Passes through any errors from the C library. See enum [`Error`](crate::Error).
    #[inline]
    pub fn intermediate_decode_with_metadata(&self, num_results: u32) -> crate::Result<Metadata> {
        let ptr =
            unsafe { coqui_stt_sys::STT_IntermediateDecodeWithMetadata(self.state, num_results) };

        if ptr.is_null() {
            return Err(crate::Error::Unknown);
        }

        Ok(crate::Metadata::new(ptr))
    }

    /// **EXPERIMENTAL**: Compute the intermediate decoding of an ongoing streaming inference,
    /// flushing buffers first.
    ///
    /// This ensures that all audio that has been streamed so far is included in the result,
    /// but is more expensive than [`intermediate_decode`](Stream::intermediate_decode)
    /// because buffers are processed through the acoustic model.
    ///
    /// Calling this function too often will also degrade transcription accuracy due to
    /// trashing of the LSTM hidden state vectors.
    ///
    /// # Errors
    /// Passes through any errors from the C library. See enum [`Error`](crate::Error).
    #[allow(clippy::missing_inline_in_public_items)]
    pub fn intermediate_decode_with_buffer_flush(&self) -> crate::Result<String> {
        let ptr = unsafe { coqui_stt_sys::STT_IntermediateDecodeFlushBuffers(self.state) };

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

    /// **EXPERIMENTAL**: Compute the intermediate decoding of an ongoing streaming inference,
    /// flushing buffers first.
    ///
    /// This ensures that all audio that has been streamed so far is included in the result,
    /// but is more expensive than
    /// [`intermediate_decode_with_metadata`](Stream::intermediate_decode_with_metadata)
    /// because buffers are processed through the acoustic model.
    ///
    /// Calling this function too often will also degrade transcription accuracy due to
    /// trashing of the LSTM hidden state vectors.
    ///
    /// Returns results including metadata.
    ///
    /// # Errors
    /// Passes through any errors from the C library. See enum [`Error`](crate::Error).
    #[inline]
    pub fn intermediate_decode_with_metadata_and_buffer_flush(
        &self,
        num_results: u32,
    ) -> crate::Result<Metadata> {
        let ptr = unsafe {
            coqui_stt_sys::STT_IntermediateDecodeWithMetadataFlushBuffers(self.state, num_results)
        };

        if ptr.is_null() {
            return Err(crate::Error::Unknown);
        }

        Ok(crate::Metadata::new(ptr))
    }

    /// Compute the final decoding of an ongoing streaming inference and
    /// return the result.
    /// Signals the end of an ongoing streaming inference.
    ///
    /// Destroys this stream object.
    ///
    /// # Errors
    /// Passes through any errors from the C library. See enum [`Error`](crate::Error).
    #[allow(clippy::missing_inline_in_public_items)]
    pub fn finish_stream(self) -> crate::Result<String> {
        let this = ManuallyDrop::new(self);

        let ptr = unsafe { coqui_stt_sys::STT_FinishStream(this.state) };

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

    /// Compute the final decoding of an ongoing streaming inference
    /// and return results including metadata.
    /// Signals the end of an ongoing streaming inference.
    ///
    /// Destroys this stream object.
    ///
    /// `num_results` is the maximum number of possible transcriptions to return.
    /// Note that it is not guaranteed this many will be returned at minimum,
    /// but there will never be more than this number at maximum.
    ///
    /// # Errors
    /// Passes through any errors from the C library. See enum [`Error`](crate::Error).
    #[inline]
    pub fn finish_stream_with_metadata(self, num_results: u32) -> crate::Result<Metadata> {
        let this = ManuallyDrop::new(self);

        let ptr =
            unsafe { coqui_stt_sys::STT_IntermediateDecodeWithMetadata(this.state, num_results) };

        if ptr.is_null() {
            return Err(crate::Error::Unknown);
        }

        Ok(crate::Metadata::new(ptr))
    }
}
