use thiserror::Error;

/// The main error type for errors returned by this crate.
#[derive(Error, Debug)]

pub enum YabaiError {
    #[error("IO Error: {0}")]
    FormatError(String),
    #[error("CommandError: {command:?} caused {message:?}")]
    CommandError { command: String, message: String },
}
