use std::error::Error as StdError;
use std::ffi::FromBytesWithNulError;
use std::fmt::{Debug, Display, Formatter};
use std::string::FromUtf8Error;

/// Type alias of the standard [Result] type to this crate's [Error] type
pub type Result<T> = std::result::Result<T, Error>;

/// All possible errors returned by the C API plus some Rust errors.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Missing model information.
    NoModel,

    /// Invalid alphabet embedded in model. (Data corruption?)
    InvalidAlphabet,
    /// Invalid model shape.
    InvalidShape,
    /// Invalid scorer file.
    InvalidScorer,
    /// Incompatible model.
    ModelIncompatible,
    /// External scorer is not enabled.
    ScorerNotEnabled,
    /// Could not read scorer file.
    ScorerUnreadable,
    /// Could not recognize language model header in scorer.
    ScorerInvalidHeader,
    /// Reached end of scorer file before loading vocabulary trie.
    ScorerNoTrie,
    /// Invalid magic in trie header.
    ScorerInvalidTrie,
    /// Scorer file version does not match expected version.
    ScorerVersionMismatch,

    /// Failed to initialize memory mapped model.
    InitMmapFailed,
    /// Failed to initialize the session.
    InitSessionFailed,
    /// Interpreter failed.
    InterpreterFailed,
    /// Failed to run the session.
    RunSessionFailed,
    /// Error creating the stream.
    CreateStreamFailed,
    /// Error reading the proto buffer model file.
    ReadProtoBufFailed,
    /// Failed to create session.
    CreateSessionFailed,
    /// Could not allocate model state.
    CreateModelFailed,
    /// Could not insert hot-word.
    InsertHotWordFailed,
    /// Could not clear hot-words.
    ClearHotWordsFailed,
    /// Could not erase hot-word.
    EraseHotWordFailed,

    /// An unknown error was returned.
    Other(i32),
    /// An unknown error was returned.
    Unknown,

    /// Null bytes were found in a string passed in.
    NulBytesFound,
    /// A string returned by `libstt` contained invalid UTF-8.
    Utf8Error(FromUtf8Error),
}

impl Error {
    pub(crate) const fn from_c_int(err: std::os::raw::c_int) -> Option<Self> {
        #[allow(clippy::enum_glob_use)]
        use self::Error::*;
        match err {
            0_i32 => None,
            0x2000_i32 => Some(InvalidAlphabet),
            0x2001_i32 => Some(InvalidShape),
            0x2002_i32 => Some(InvalidScorer),
            0x2003_i32 => Some(ModelIncompatible),
            0x2004_i32 => Some(ScorerNotEnabled),
            0x2005_i32 => Some(ScorerUnreadable),
            0x2006_i32 => Some(ScorerInvalidHeader),
            0x2007_i32 => Some(ScorerNoTrie),
            0x2008_i32 => Some(ScorerInvalidTrie),
            0x2009_i32 => Some(ScorerVersionMismatch),
            0x3000_i32 => Some(InitMmapFailed),
            0x3001_i32 => Some(InitSessionFailed),
            0x3002_i32 => Some(InterpreterFailed),
            0x3003_i32 => Some(RunSessionFailed),
            0x3004_i32 => Some(CreateStreamFailed),
            0x3005_i32 => Some(ReadProtoBufFailed),
            0x3006_i32 => Some(CreateSessionFailed),
            0x3007_i32 => Some(CreateModelFailed),
            0x3008_i32 => Some(InsertHotWordFailed),
            0x3009_i32 => Some(ClearHotWordsFailed),
            0x3010_i32 => Some(EraseHotWordFailed),
            _ => Some(Other(err)),
        }
    }
}

impl Display for Error {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let fancy_err: std::borrow::Cow<str> = match self {
            Self::NoModel => "Missing model information.".into(),
            Self::InvalidAlphabet => {
                "Invalid alphabet embedded in model. (Data corruption?)".into()
            }
            Self::InvalidShape => "Invalid model shape.".into(),
            Self::InvalidScorer => "Invalid scorer file.".into(),
            Self::ModelIncompatible => "Incompatible model.".into(),
            Self::ScorerNotEnabled => "External scorer is not enabled.".into(),
            Self::ScorerUnreadable => "Could not read scorer file.".into(),
            Self::ScorerInvalidHeader => {
                "Could not recognize language model header in scorer.".into()
            }
            Self::ScorerNoTrie => {
                "Reached end of scorer file before loading vocabulary trie.".into()
            }
            Self::ScorerInvalidTrie => "Invalid magic in trie header.".into(),
            Self::ScorerVersionMismatch => {
                "Scorer file version does not match expected version.".into()
            }
            Self::InitMmapFailed => "Failed to initialize memory mapped model.".into(),
            Self::InitSessionFailed => "Failed to initialize the session.".into(),
            Self::InterpreterFailed => "Interpreter failed.".into(),
            Self::RunSessionFailed => "Failed to run the session.".into(),
            Self::CreateStreamFailed => "Error creating the stream.".into(),
            Self::ReadProtoBufFailed => "Error reading the proto buffer model file.".into(),
            Self::CreateSessionFailed => "Failed to create session.".into(),
            Self::CreateModelFailed => "Could not allocate model state.".into(),
            Self::InsertHotWordFailed => "Could not insert hot-word.".into(),
            Self::ClearHotWordsFailed => "Could not clear hot-words.".into(),
            Self::EraseHotWordFailed => "Could not erase hot-word.".into(),
            Self::Utf8Error(e) => format!(
                "A string returned by `libstt` contained invalid UTF-8: {}",
                e
            )
            .into(),
            _ => "An unknown error was returned.".into(),
        };
        f.write_str(fancy_err.as_ref())
    }
}

impl StdError for Error {}

impl From<FromBytesWithNulError> for Error {
    #[inline]
    fn from(_: FromBytesWithNulError) -> Self {
        Self::NulBytesFound
    }
}

impl From<FromUtf8Error> for Error {
    #[inline]
    fn from(e: FromUtf8Error) -> Self {
        Self::Utf8Error(e)
    }
}
