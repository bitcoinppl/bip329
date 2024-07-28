/// Errors that can occur when parsing a label.
#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum ParseError {
    #[error("Unable to read file: {0}")]
    FileReadError(String),

    #[error("Unable to parse file: {0}")]
    ParseError(String),
}

/// Errors that can occur when exporting a label.
#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum ExportError {
    #[error("Unable to write file: {0}")]
    FileWriteError(String),

    #[error("Unable to serialize labels : {0}")]
    SerializeError(String),
}

/// Errors that can occur when encrypting or decrypting a label.
#[cfg(feature = "encryption")]
#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum EncryptionError {
    #[error("Unable to encrypt labels: {0}")]
    EncryptError(String),

    #[error("Unable to encrypt labels: {0}")]
    DecryptError(String),

    #[error("Unable to parse labels: {0}")]
    ParseError(#[from] ParseError),

    #[error("Unable to export labels: {0}")]
    ExportError(#[from] ExportError),

    #[error("Unable to write labels: {0}")]
    WriteError(String),

    #[error("Decrypted to invalid UTF-8 string: {0}")]
    Utf8Error(String),

    #[error("Invalid hex encoded string: {0}")]
    HexError(String),
}

impl From<crate::error::ParseError> for ParseError {
    fn from(e: crate::error::ParseError) -> Self {
        match e {
            crate::error::ParseError::FileReadError(e) => ParseError::FileReadError(e.to_string()),
            crate::error::ParseError::ParseError(e) => ParseError::ParseError(e.to_string()),
        }
    }
}

impl From<crate::error::ExportError> for ExportError {
    fn from(e: crate::error::ExportError) -> Self {
        match e {
            crate::error::ExportError::FileWriteError(e) => {
                ExportError::FileWriteError(e.to_string())
            }
            crate::error::ExportError::SerializeError(e) => {
                ExportError::SerializeError(e.to_string())
            }
        }
    }
}

#[cfg(feature = "encryption")]
impl From<crate::error::EncryptionError> for EncryptionError {
    fn from(e: crate::error::EncryptionError) -> Self {
        match e {
            crate::error::EncryptionError::EncryptError(e) => {
                EncryptionError::EncryptError(e.to_string())
            }
            crate::error::EncryptionError::DecryptError(e) => {
                EncryptionError::DecryptError(e.to_string())
            }
            crate::error::EncryptionError::ParseError(e) => EncryptionError::ParseError(e.into()),
            crate::error::EncryptionError::ExportError(e) => EncryptionError::ExportError(e.into()),
            crate::error::EncryptionError::WriteError(e) => {
                EncryptionError::WriteError(e.to_string())
            }
            crate::error::EncryptionError::Utf8Error(e) => {
                EncryptionError::Utf8Error(e.to_string())
            }
            crate::error::EncryptionError::HexError(e) => EncryptionError::HexError(e.to_string()),
        }
    }
}
