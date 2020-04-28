use crate::framework::types::CommandError;

/// The common result type between most library functions.
pub type Result<T> = std::result::Result<T, Error>;

/// A common error enum returned by most of the library's functionality
pub enum Error {
    /// An error generated within this library
    Telegram(TelegramError),
    /// An error from the `hyper` crate.
    Hyper(hyper::Error),
    /// An std::io error.
    IO(std::io::Error),
    /// An error from the `http` crate.
    HTTP(http::Error),
    /// An error from the `serde_json` crate.
    JSON(serde_json::Error),
    /// An error happened in a command
    Command(CommandError),
}

/// An error enum returned by errors generated within the library itself
pub enum TelegramError {
    NoToken,
    InvalidToken,
    MissingPermission,
    NotFound,
    ServerError,
    InvalidEndpoint,
    InvalidCommandType,
    InvalidArgument(String),
    APIResponseError(String),
    Unknown(String),
}

impl TelegramError {
    pub fn description(&self) -> String {
        match *self {
            TelegramError::NoToken => "No token provided to login to telegram".to_owned(),
            TelegramError::InvalidToken => {
                "Invalid token provided for logging in to telegram".to_owned()
            },
            TelegramError::MissingPermission => {
                "Missing permission to execute action in chat".to_owned()
            },
            TelegramError::NotFound => "The requested resource doesn't exist".to_owned(),
            TelegramError::ServerError => {
                "The telegram server returned a 500 status code".to_owned()
            },
            TelegramError::InvalidEndpoint => "The requested endpoint does not exist".to_owned(),
            TelegramError::InvalidCommandType => {
                "This action cannot be done on this command type".to_owned()
            },
            TelegramError::InvalidArgument(ref e) => format!("Invalid argument provided: {}", e),
            TelegramError::APIResponseError(ref e) => {
                format!("the telegram api returned an error: {}", e)
            },
            TelegramError::Unknown(ref e) => format!("unknown error occurred: {}", e),
        }
    }
}

impl std::fmt::Display for TelegramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.description().as_str())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Telegram(e) => std::fmt::Display::fmt(&e, f),
            Error::Hyper(e) => std::fmt::Display::fmt(&e, f),
            Error::IO(e) => std::fmt::Display::fmt(&e, f),
            Error::HTTP(e) => std::fmt::Display::fmt(&e, f),
            Error::JSON(e) => std::fmt::Display::fmt(&e, f),
            Error::Command(e) => std::fmt::Display::fmt(&e.0, f),
        }
    }
}

impl std::fmt::Debug for TelegramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("telegram::Error")
            .field(&self.description())
            .finish()
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Telegram(e) => std::fmt::Debug::fmt(&e, f),
            Error::Hyper(e) => std::fmt::Debug::fmt(&e, f),
            Error::IO(e) => std::fmt::Debug::fmt(&e, f),
            Error::HTTP(e) => std::fmt::Debug::fmt(&e, f),
            Error::JSON(e) => std::fmt::Debug::fmt(&e, f),
            Error::Command(e) => std::fmt::Debug::fmt(&e, f),
        }
    }
}

impl std::error::Error for TelegramError {}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            Error::Telegram(e) => e,
            Error::Hyper(e) => e,
            Error::IO(e) => e,
            Error::HTTP(e) => e,
            Error::JSON(e) => e,
            Error::Command(_) => return None,
        })
    }
}

impl From<TelegramError> for Error {
    fn from(e: TelegramError) -> Self {
        Self::Telegram(e)
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        Self::Hyper(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<http::Error> for Error {
    fn from(e: http::Error) -> Self {
        Self::HTTP(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::JSON(e)
    }
}
