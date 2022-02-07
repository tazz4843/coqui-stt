use std::borrow::{Borrow, Cow};
use std::ffi::CStr;
use std::fmt::{Debug, Display, Formatter};

/// Stores text of an individual token, along with its timing information.
#[repr(transparent)]
pub struct TokenMetadata {
    ptr: coqui_stt_sys::TokenMetadata,
}

unsafe impl Send for TokenMetadata {}
unsafe impl Sync for TokenMetadata {}

impl TokenMetadata {
    /// The text corresponding to this token
    #[inline]
    #[must_use]
    pub fn text(&self) -> Cow<str> {
        // SAFETY: self.ptr.text will always point to valid metadata
        let cstr = unsafe { CStr::from_ptr(self.ptr.text) };
        cstr.to_string_lossy()
    }

    /// Position of the token in units of 20ms
    #[inline]
    #[must_use]
    pub const fn timestep(&self) -> u32 {
        self.ptr.timestep
    }

    /// Position of the token in seconds
    #[inline]
    #[must_use]
    pub const fn start_time(&self) -> f32 {
        self.ptr.start_time
    }

    /// Convert this into an [`OwnedTokenMetadata`](OwnedTokenMetadata) struct.
    ///
    /// This is relatively cheap compared to its parent `to_owned` functions,
    /// as it needs to clone just a `String` and two numbers.
    #[inline]
    #[must_use]
    pub fn to_owned(&self) -> OwnedTokenMetadata {
        let coqui_stt_sys::TokenMetadata {
            timestep,
            start_time,
            ..
        } = self.ptr;

        let text = self.text().to_string();

        OwnedTokenMetadata {
            text,
            timestep,
            start_time,
        }
    }
}

impl Debug for TokenMetadata {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenMetadata")
            .field("text", &self.text())
            .field("timestep", &self.timestep())
            .field("start_time", &self.start_time())
            .finish()
    }
}

impl Display for TokenMetadata {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.text().borrow())
    }
}

/// An owned variant of [`TokenMetadata`](TokenMetadata).
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct OwnedTokenMetadata {
    /// The text corresponding to this token
    pub text: String,
    /// Position of the token in units of 20ms
    pub timestep: u32,
    /// Position of the token in seconds
    pub start_time: f32,
}

impl Display for OwnedTokenMetadata {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.text)
    }
}
