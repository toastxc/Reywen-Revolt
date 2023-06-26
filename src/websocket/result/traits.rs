use tokio_tungstenite::tungstenite;

use super::error::WSError;

pub trait ErrorConvert<T: std::fmt::Debug> {
    fn res(self) -> Result<T, WSError>;
}
impl<T: std::fmt::Debug> ErrorConvert<T> for Result<T, tungstenite::Error> {
    fn res(self) -> Result<T, WSError> {
        match self {
            Ok(data) => Ok(data),
            Err(error) => Err(WSError::Tungstenite(error)),
        }
    }
}
impl<T: std::fmt::Debug> ErrorConvert<T> for Result<T, std::io::Error> {
    fn res(self) -> Result<T, WSError> {
        match self {
            Ok(data) => Ok(data),
            Err(error) => Err(WSError::Connection(error)),
        }
    }
}
impl<T: std::fmt::Debug> ErrorConvert<T> for Result<T, serde_json::Error> {
    fn res(self) -> Result<T, WSError> {
        match self {
            Ok(data) => Ok(data),
            Err(error) => Err(WSError::Serde(error)),
        }
    }
}

/*


use tokio_tungstenite::tungstenite;

#[derive(Debug)]
pub enum WSError {
    Serde(serde_json::Error),
    Tungstenite(tungstenite::Error),
    Data(DataError),
    Connection(std::io::Error)
}
#[derive(Debug)]
pub enum DataError {
    DataTypeNotFound,
}


*/
