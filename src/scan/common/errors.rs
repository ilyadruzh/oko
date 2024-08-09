// use crate::scan::bitcoin::blockchain::proto::script;
use rusty_leveldb::Status;
use std::convert::From;
use std::error;
use std::fmt;
use std::io;
use std::string;
use std::sync;

// macro_rules! line_mark {
//     () => {
//         format!("Marked line: {} @ {}:{}", file!(), line!(), column!())
//     };
// }

// /// Transforms a Option to Result
// /// If the Option contains None, a line mark will be placed along with OkoErrorKind::None
// macro_rules! transform {
//     ($e:expr) => {{
//         $e.ok_or(OkoError::new(OkoErrorKind::None).join_msg(&line_mark!()))?
//     }};
// }

pub type OkoResult<T> = Result<T, OkoError>;

#[derive(Debug)]
pub struct OkoError {
    pub kind: OkoErrorKind,
    pub message: String,
}

impl OkoError {
    pub fn new(kind: OkoErrorKind) -> Self {
        OkoError {
            kind,
            message: String::new(),
        }
    }

    /// Joins the Error with a new message and returns it
    pub fn join_msg(mut self, msg: &str) -> Self {
        self.message.push_str(msg);
        OkoError {
            kind: self.kind,
            message: self.message,
        }
    }
}

impl fmt::Display for OkoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.message.is_empty() {
            write!(f, "{}", &self.kind)
        } else {
            write!(f, "{} {}", &self.message, &self.kind)
        }
    }
}

impl error::Error for OkoError {
    fn description(&self) -> &str {
        self.message.as_ref()
    }
    fn cause(&self) -> Option<&dyn error::Error> {
        self.kind.source()
    }
}

#[derive(Debug)]
pub enum OkoErrorKind {
    None,
    IoError(io::Error),
    ByteOrderError(io::Error),
    Utf8Error(string::FromUtf8Error),
    // ScriptError(crate::scan::common::errors::OkoErrorKind),
    InvalidArgsError,
    CallbackError,
    ValidationError,
    RuntimeError,
    PoisonError,
    SendError,
    LevelDBError(String),
}

impl fmt::Display for OkoErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OkoErrorKind::IoError(ref err) => write!(f, "I/O Error: {}", err),
            OkoErrorKind::ByteOrderError(ref err) => write!(f, "ByteOrder: {}", err),
            OkoErrorKind::Utf8Error(ref err) => write!(f, "Utf8 Conversion: {}", err),
            // OkoErrorKind::ScriptError(ref err) => write!(f, "Script: {}", err),
            OkoErrorKind::LevelDBError(ref err) => write!(f, "LevelDB: {}", err),
            ref err @ OkoErrorKind::PoisonError => write!(f, "Threading Error: {}", err),
            ref err @ OkoErrorKind::SendError => write!(f, "Sync: {}", err),
            ref err @ OkoErrorKind::InvalidArgsError => write!(f, "InvalidArgs: {}", err),
            ref err @ OkoErrorKind::CallbackError => write!(f, "Callback: {}", err),
            ref err @ OkoErrorKind::ValidationError => write!(f, "Validation: {}", err),
            ref err @ OkoErrorKind::RuntimeError => write!(f, "RuntimeError: {}", err),
            OkoErrorKind::None => write!(f, ""),
        }
    }
}

impl error::Error for OkoErrorKind {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            OkoErrorKind::IoError(ref err) => Some(err),
            OkoErrorKind::ByteOrderError(ref err) => Some(err),
            OkoErrorKind::Utf8Error(ref err) => Some(err),
            // OkoErrorKind::ScriptError(ref err) => Some(err),
            ref err @ OkoErrorKind::PoisonError => Some(err),
            ref err @ OkoErrorKind::SendError => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for OkoError {
    fn from(err: io::Error) -> Self {
        Self::new(OkoErrorKind::IoError(err))
    }
}

impl From<i32> for OkoError {
    fn from(err_code: i32) -> Self {
        Self::from(io::Error::from_raw_os_error(err_code))
    }
}

impl From<String> for OkoError {
    fn from(err: String) -> Self {
        Self::new(OkoErrorKind::None).join_msg(&err)
    }
}

impl<T> From<sync::PoisonError<T>> for OkoError {
    fn from(_: sync::PoisonError<T>) -> Self {
        Self::new(OkoErrorKind::PoisonError)
    }
}

impl<T> From<sync::mpsc::SendError<T>> for OkoError {
    fn from(_: sync::mpsc::SendError<T>) -> Self {
        Self::new(OkoErrorKind::SendError)
    }
}

impl From<string::FromUtf8Error> for OkoError {
    fn from(err: string::FromUtf8Error) -> Self {
        Self::new(OkoErrorKind::Utf8Error(err))
    }
}

impl From<rusty_leveldb::Status> for OkoError {
    fn from(status: Status) -> Self {
        Self::new(OkoErrorKind::LevelDBError(status.err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_op_error() {
        let kind = io::Error::new(io::ErrorKind::BrokenPipe, "oh no!");
        let err = OkoError::from(kind);
        assert_eq!(format!("{}", err), "I/O Error: oh no!");

        let err = err.join_msg("Cannot proceed.");
        assert_eq!(format!("{}", err), "Cannot proceed. I/O Error: oh no!");
    }
}
