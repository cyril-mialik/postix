use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum ApiError {
    Unauthorized,
    Message(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unauthorized => {
                formatter.write_str("Your session has expired. Please sign in again.")
            }
            Self::Message(message) => formatter.write_str(message),
        }
    }
}
