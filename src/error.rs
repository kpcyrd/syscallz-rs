use std::ffi::CStr;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Os(i32),
    Msg(String),
}

pub type Result<T> = std::result::Result<T, Error>;

// https://github.com/rust-lang/rust/blob/e649e903440bfd919bfc9db848c28df6d795a116/src/libstd/sys/unix/os.rs#L91
fn errno_string(errno: i32) -> String {
    const TMPBUF_SZ: usize = 128;
    let mut buf = [0 as libc::c_char; TMPBUF_SZ];
    let c_str = unsafe {
        if libc::strerror_r(errno, buf.as_mut_ptr(), buf.len()) < 0 {
            panic!("strerror_r failure")
        }
        CStr::from_ptr(buf.as_ptr())
    };
    c_str.to_str().unwrap().to_owned()
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Os(errno) => f.write_str(&errno_string(errno)),
            Error::Msg(ref msg) => f.write_str(msg),
        }
    }
}

impl Error {
    #[inline]
    pub fn from_errno(errno: i32) -> Self {
        Error::Os(errno)
    }

    #[inline]
    pub fn errno(&self) -> Option<i32> {
        match *self {
            Error::Os(errno) => Some(errno),
            Error::Msg(_) => None,
        }
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Error {
        Error::Msg(msg)
    }
}

impl std::error::Error for Error {
    /// This method is soft-deprecated.
    fn description(&self) -> &str {
        match *self {
            Error::Os(_) => "seccomp os error; use Display",
            Error::Msg(ref msg) => msg,
        }
    }
}
