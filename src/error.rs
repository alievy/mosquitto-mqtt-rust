/// A specialized result type
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// The kinds of errors that can occur.
#[derive(Debug)]
pub enum Error {
    /// MosquittoInitError.
    MosquittoInit,

    /// MosquittoNull error.
    MosquittoNull,

    /// MosquittoNewInstance error.
    MosquittoNewInstance,

    /// MosquittoCleanup error.
    MosquittoCleanup(i32),

    /// MosquittoUserPass error.
    MosquittoUserPass(i32),

    /// MosquittoConnect error.
    MosquittoConnect(i32),

    /// MosquittoDisconnect error.
    MosquittoDisconnect(i32),

    /// MosquittoReconnect error.
    MosquittoReconnect(i32),

    /// MosquittoPublish error.
    MosquittoPublish(i32),

    /// MosquittoSubscribe error.
    MosquittoSubscribe(i32),

    /// MosquittoUnsubscribe error.
    MosquittoUnsubscribe(i32),

    /// MosquittoMqttLoop error.
    MosquittoMqttLoop(i32),

    /// MosquittoMqttLoopStart error.
    MosquittoMqttLoopStart(i32),

    /// MosquittoLoopWrite error.
    MosquittoLoopWrite(i32),

    /// MosquittoLoopMisc error.
    MosquittoLoopMisc(i32),

    /// MosquittoTlsSet error.
    MosquittoTlsSet(i32),

    /// Any boxed error.
    Boxed(Box<dyn std::error::Error>),

    /// I/O error.
    Io(std::io::Error),

    /// ParseIntError.
    ParseInt(std::num::ParseIntError),

    /// NulError.
    NulError(std::ffi::NulError),

    /// MosquittoSocket error.
    MosquittoSocket,

    /// CString error.
    CString,

    /// Unexpected Behaviour
    UnexpectedBehaviour,
}

impl From<Box<dyn std::error::Error>> for Error {
    /// A source containing any boxed error.
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Error::Boxed(error)
    }
}

impl From<std::io::Error> for Error {
    /// A source containing an I/O error.
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::ParseInt(err)
    }
}

impl From<std::ffi::NulError> for Error {
    /// A source containing a nul error.
    fn from(error: std::ffi::NulError) -> Self {
        Error::NulError(error)
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    /// The printed representation of an error kind.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::UnexpectedBehaviour => write!(f, "Unexpected behaviour"),
            Error::MosquittoInit => write!(f, "MosquittoInit error"),
            Error::MosquittoNull => write!(f, "MosquittoNull error"),
            Error::MosquittoNewInstance => write!(f, "MosquittoNewInstance error"),
            Error::MosquittoCleanup(status_code) => {
                write!(f, "MosquittoCleanup error - status code: {}", status_code)
            }
            Error::MosquittoUserPass(status_code) => {
                write!(f, "MosquittoUserPass error - status code: {}", status_code)
            }
            Error::MosquittoConnect(status_code) => {
                write!(f, "MosquittoConnect error - status code: {}", status_code)
            }
            Error::MosquittoDisconnect(status_code) => write!(
                f,
                "MosquittoDisconnect error - status code: {}",
                status_code
            ),
            Error::MosquittoReconnect(status_code) => {
                write!(f, "MosquittoReconnect error - status code: {}", status_code)
            }
            Error::MosquittoPublish(status_code) => {
                write!(f, "MosquittoPublish error - status code: {}", status_code)
            }
            Error::MosquittoSubscribe(status_code) => {
                write!(f, "MosquittoSubscribe error - status code: {}", status_code)
            }
            Error::MosquittoUnsubscribe(status_code) => write!(
                f,
                "MosquittoUnsubscribe error - status code: {}",
                status_code
            ),
            Error::MosquittoMqttLoop(status_code) => {
                write!(f, "MosquittoMqttLoop error - status code: {}", status_code)
            }
            Error::MosquittoMqttLoopStart(status_code) => write!(
                f,
                "MosquittoMqttLoopStart error - status code: {}",
                status_code
            ),
            Error::MosquittoLoopWrite(status_code) => {
                write!(f, "MosquittoLoopWrite error - status code: {}", status_code)
            }
            Error::MosquittoLoopMisc(status_code) => {
                write!(f, "MosquittoLoopMisc error - status code: {}", status_code)
            }
            Error::MosquittoSocket => write!(f, "MosquittoSocket error"),
            Error::MosquittoTlsSet(status_code) => {
                write!(f, "MosquittoTlsSet error - status code: {}", status_code)
            }
            Error::CString => write!(f, "CString error"),
            Error::Boxed(error) => write!(f, "{}", error),
            Error::Io(error) => write!(f, "{}", error),
            Error::ParseInt(error) => write!(f, "{}", error),
            Error::NulError(error) => write!(f, "{}", error),
        }
    }
}
