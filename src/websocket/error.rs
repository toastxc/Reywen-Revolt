use tokio_tungstenite::tungstenite;
use url::ParseError;

#[derive(Debug)]
pub enum Error {
    Serde(serde_json::Error),
    Tungstenite(tungstenite::Error),
    Data(DataError),
    Connection(std::io::Error),
    Watchdog(WatchdogError),
    Url(ParseError)
}
#[derive(Debug)]
pub enum DataError {
    DataTypeNotFound,
}
#[derive(Debug)]
pub enum WatchdogError {
    ExceededComplianceLimit,
}

impl From<tokio_tungstenite::tungstenite::Error> for Error {
    fn from(value: tungstenite::Error) -> Self {
        Self::Tungstenite(value)
    }
}

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Self::Url(value)
    }
}