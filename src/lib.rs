//! Package implement peer id specification.

use std::{error, fmt, result};

use multiformats;

/// Short form to compose Error values.
///
/// Here are few possible ways:
///
/// ```ignore
/// use crate::Error;
/// err_at!(Invalid, msg: format!("bad argument"));
/// ```
///
/// ```ignore
/// use crate::Error;
/// err_at!(Invalid, std::io::read(buf));
/// ```
///
/// ```ignore
/// use crate::Error;
/// err_at!(Invalid, std::fs::read(file_path), format!("read failed"));
/// ```
///
#[macro_export]
macro_rules! err_at {
    ($v:ident, msg: $($arg:expr),+) => {{
        let prefix = format!("{}:{}", file!(), line!());
        Err(Error::$v(prefix, format!($($arg),+)))
    }};
    ($v:ident, $e:expr) => {{
        match $e {
            Ok(val) => Ok(val),
            Err(err) => {
                let prefix = format!("{}:{}", file!(), line!());
                Err(Error::$v(prefix, format!("{}", err)))
            }
        }
    }};
    ($v:ident, $e:expr, $($arg:expr),+) => {{
        match $e {
            Ok(val) => Ok(val),
            Err(err) => {
                let prefix = format!("{}:{}", file!(), line!());
                let msg = format!($($arg),+);
                Err(Error::$v(prefix, format!("{} {}", err, msg)))
            }
        }
    }};
}

pub mod identity;
mod pb;
mod peer_id;
pub use peer_id::PeerId;

/// Type alias for Result return type, used by this package.
pub type Result<T> = result::Result<T, Error>;

/// Error variants that can be returned by this package's API.
///
/// Each variant carries a prefix, typically identifying the
/// error location.
pub enum Error {
    Invalid(String, String),
    DecodeError(String, String),
    EncodeError(String, String),
    SigningError(String, String),
    BadInput(String, String),
    FailMultiformat(String, String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        use Error::*;

        match self {
            Invalid(p, msg) => write!(f, "{} Invalid: {}", p, msg),
            DecodeError(p, msg) => write!(f, "{} DecodeError: {}", p, msg),
            EncodeError(p, msg) => write!(f, "{} EncodeError: {}", p, msg),
            SigningError(p, msg) => write!(f, "{} SigningError: {}", p, msg),
            BadInput(p, msg) => write!(f, "{} BadInput: {}", p, msg),
            FailMultiformat(_, msg) => write!(f, "{}", msg),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl error::Error for Error {}

impl From<multiformats::Error> for Error {
    fn from(err: multiformats::Error) -> Error {
        Error::FailMultiformat("".to_string(), err.to_string())
    }
}
