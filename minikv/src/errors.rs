use std::result;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Errors {
    #[error("failed to read data from file")]
    FailedToReadDataFromFile,
    #[error("failed to write data to file")]
    FailedToWriteDataToFile,
    #[error("failed to sync data file")]
    FailedToSyncDataFile,
    #[error("failed to open data file")]
    FailedToOpenDataFile,
}

/// 重写系统 result 类型，以满足自定义错误类型的需要
pub type Result<T> = result::Result<T, Errors>;
