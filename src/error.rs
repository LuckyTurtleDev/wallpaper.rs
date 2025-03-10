use std::{io, string::FromUtf16Error, string::FromUtf8Error};
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("I/O Error: {0}")]
    IOError(#[from] io::Error),

    #[error("Invalid UTF-8: {0}")]
    InvalidUtf8_8(#[from] FromUtf8Error),

    #[error("Invalid UTF-8: {0}")]
    InvalidUtf8_16(#[from] FromUtf16Error),

    #[cfg(all(unix, not(target_os = "macos")))]
    #[error("Invalid INI: {0}")]
    InvalidIni(#[from] ini::ini::Error),

    #[cfg(unix)]
    #[error("Enquote error: {0}")]
    Enquote(#[from] enquote::Error),

    #[error("{command} exited with status code {code}")]
    CommandFailed { command: String, code: i32 },

    #[error("Could not find config directory")]
    NoConfigDir,

    #[error("No {0} image found")]
    NoImage(&'static str),

    #[error("Unsupported Desktop")]
    UnsupportedDesktop,
}
