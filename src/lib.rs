//! Package implement multiformat specifications.

#![feature(box_syntax, box_patterns)]

use std::{error, fmt, result};

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

#[macro_use]
extern crate data_encoding_macro;

pub mod multiaddr;
pub mod multibase;
pub mod multicodec;
pub mod multihash;

/// Type alias for Result return type, used by this package.
pub type Result<T> = result::Result<T, Error>;

/// Error variants that can be returned by this package's API.
///
/// Each variant carries a prefix, typically identifying the
/// error location.
pub enum Error {
    Fatal(String, String),
    FailConvert(String, String),
    IOError(String, String),
    SysFail(String, String),
    IPCFail(String, String),
    IndexFail(String, String),
    FailCbor(String, String),
    ThreadFail(String, String),
    FilePath(String, String),
    Invalid(String, String),
    ParseError(String, String),
    DecodeError(String, String),
    EncodeError(String, String),
    DnsError(String, String),
    SigningError(String, String),
    BadInput(String, String),
    BadCodec(String, String),
    BadAddr(String, String),
    HashFail(String, String),
    NotImplemented(String, String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        use Error::*;

        match self {
            Fatal(p, msg) => write!(f, "{} Fatal: {}", p, msg),
            FailConvert(p, msg) => write!(f, "{} FailConvert: {}", p, msg),
            IOError(p, msg) => write!(f, "{} IOError: {}", p, msg),
            SysFail(p, msg) => write!(f, "{} SysFail: {}", p, msg),
            IPCFail(p, msg) => write!(f, "{} IPCFail: {}", p, msg),
            IndexFail(p, msg) => write!(f, "{} IndexFail: {}", p, msg),
            FailCbor(p, msg) => write!(f, "{} FailCbor: {}", p, msg),
            ThreadFail(p, msg) => write!(f, "{} ThreadFail: {}", p, msg),
            FilePath(p, msg) => write!(f, "{} FilePath: {}", p, msg),
            Invalid(p, msg) => write!(f, "{} Invalid: {}", p, msg),
            ParseError(p, msg) => write!(f, "{} ParseError: {}", p, msg),
            DecodeError(p, msg) => write!(f, "{} DecodeError: {}", p, msg),
            EncodeError(p, msg) => write!(f, "{} EncodeError: {}", p, msg),
            DnsError(p, msg) => write!(f, "{} DnsError: {}", p, msg),
            SigningError(p, msg) => write!(f, "{} SigningError: {}", p, msg),
            BadInput(p, msg) => write!(f, "{} BadInput: {}", p, msg),
            BadCodec(p, msg) => write!(f, "{} BadCodec: {}", p, msg),
            BadAddr(p, msg) => write!(f, "{} BadAddr: {}", p, msg),
            HashFail(p, msg) => write!(f, "{} HashFail: {}", p, msg),
            NotImplemented(p, msg) => write!(f, "{} NotImplemented: {}", p, msg),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl error::Error for Error {}
