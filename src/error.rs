#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unable to read file: {0}")]
    FileReadError(#[from] std::io::Error),

    #[error("Unable to parse file: {0}")]
    ParseError(#[from] serde_json::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("Unable to write file: {0}")]
    FileWriteError(#[from] std::io::Error),

    #[error("Unable to serialize labels : {0}")]
    SerializeError(#[from] serde_json::Error),
}
