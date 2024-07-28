#[cfg(feature = "uniffi")]
pub mod ffi;

/// Errors that can occur when parsing a label.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unable to read file: {0}")]
    FileReadError(#[from] std::io::Error),

    #[error("Unable to parse file: {0}")]
    ParseError(#[from] serde_json::Error),
}

/// Errors that can occur when exporting a label.
#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("Unable to write file: {0}")]
    FileWriteError(#[from] std::io::Error),

    #[error("Unable to serialize labels : {0}")]
    SerializeError(#[from] serde_json::Error),
}

/// Errors that can occur when encrypting or decrypting a label.
#[cfg(feature = "encryption")]
#[derive(Debug, thiserror::Error)]
pub enum EncryptionError {
    #[error("Unable to encrypt labels: {0}")]
    EncryptError(#[from] age::EncryptError),

    #[error("Unable to encrypt labels: {0}")]
    DecryptError(#[from] age::DecryptError),

    #[error("Unable to parse labels: {0}")]
    ParseError(#[from] ParseError),

    #[error("Unable to export labels: {0}")]
    ExportError(#[from] ExportError),

    #[error("Unable to write labels: {0}")]
    WriteError(#[from] std::io::Error),

    #[error("Decrypted to invalid UTF-8 string: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("Invalid hex encoded string: {0}")]
    HexError(#[from] hex::FromHexError),
}
