use tokio_tungstenite::tungstenite;

#[derive(Debug)]
pub enum WSError {
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
