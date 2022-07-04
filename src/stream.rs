use crate::{Metadata, Model};
use std::ffi::CStr;

/// Streaming inference state.
pub struct Stream<'a> {
    pub(crate) model: &'a mut Model,
    pub(crate) state: *mut coqui_stt_sys::StreamingState,
    /// True if this state has already been freed.
    /// This is used to prevent double-freeing.
    pub(crate) already_freed: bool,
}

// NOTE:
// Streams are thread-safe, with one major caveat:
// they cannot be used from multiple threads concurrently.
// The only reason we can safely implement Sync is because
// the compiler statically enforces that this is used from
// only one thread at a time with mutable references on all
// functions that access the C API.
unsafe impl Send for Stream<'_> {}
unsafe impl Sync for Stream<'_> {}

impl Drop for Stream<'_> {
    #[inline]
    fn drop(&mut self) {
        if !self.already_freed {
            unsafe { coqui_stt_sys::STT_FreeStream(self.state) }
        }
    }
}

impl<'a> Stream<'a> {
    /// Create a new `Stream` from a [`Model`](Model).
    ///
    /// # Errors
    /// Passes through any errors from the C library. See enum [`Error`](crate::Error).
    #[allow(clippy::missing_inline_in_public_items)]
    pub fn from_model(model: &'a mut Model) -> crate::Result<Stream<'a>> {
        let mut state = std::ptr::null_mut::<coqui_stt_sys::StreamingState>();

        let retval =
            unsafe { coqui_stt_sys::STT_CreateStream(model.0, std::ptr::addr_of_mut!(state)) };

        if let Some(e) = crate::Error::from_c_int(retval) {
            return Err(e);
        }

        if state.is_null() {
            return Err(crate::Error::Unknown);
        }

        Ok(Self {
            model,
            state,
            already_freed: false,
        })
    }

    /// Get the inner pointer to the [`StreamingState`](coqui_stt_sys::StreamingState)
    /// of this `Stream`.
    ///
    /// # Safety
    /// Once this is called, the memory management of the `Stream` is no longer handled for you.
    ///
    /// The [`Model`] object this stream points to must not be disposed of until this `Stream` is disposed of.
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
    pub unsafe fn into_state(mut self) -> *mut coqui_stt_sys::StreamingState {
        self.already_freed = true;
        self.state
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
    pub unsafe fn from_ptr(
        model: &'a mut Model,
        state: *mut coqui_stt_sys::StreamingState,
    ) -> Stream<'a> {
        Self {
            model,
            state,
            already_freed: false,
        }
    }

    /// Return a reference to the [`Model`](crate::Model) this `Stream` references.
    #[inline]
    #[must_use]
    pub fn model(&self) -> &Model {
        self.model
    }

    /// Return a mutable reference to the [`Model`](crate::Model) this wraps.
    #[inline]
    #[must_use]
    pub fn model_mut(&mut self) -> &mut Model {
        self.model
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
    pub fn intermediate_decode(&mut self) -> crate::Result<String> {
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
    pub fn intermediate_decode_with_metadata(
        &mut self,
        num_results: u32,
    ) -> crate::Result<Metadata> {
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
    pub fn intermediate_decode_with_buffer_flush(&mut self) -> crate::Result<String> {
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
        &mut self,
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
    pub fn finish_stream(mut self) -> crate::Result<String> {
        let ptr = unsafe { coqui_stt_sys::STT_FinishStream(self.state) };

        self.already_freed = true;

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
    pub fn finish_stream_with_metadata(mut self, num_results: u32) -> crate::Result<Metadata> {
        let ptr = unsafe { coqui_stt_sys::STT_FinishStreamWithMetadata(self.state, num_results) };

        self.already_freed = true;

        if ptr.is_null() {
            return Err(crate::Error::Unknown);
        }

        Ok(crate::Metadata::new(ptr))
    }
}
