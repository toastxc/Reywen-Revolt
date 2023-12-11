use tokio_tungstenite::tungstenite;

#[derive(Debug)]
pub enum Error {
    Serde(serde_json::Error),
    Tungstenite(tungstenite::Error),
    Data(DataError),
    Connection(std::io::Error),
    Watchdog(WatchdogError),
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
