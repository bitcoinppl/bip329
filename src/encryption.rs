//! Module for encrypting and decrypting labels.

use std::{
    io::{Read as _, Write as _},
    path::Path,
};

use age::secrecy::Secret;
use serde::{Deserialize, Serialize};

use crate::{error::EncryptionError, Labels};

/// A list of encrypted labels.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EncryptedLabels(Vec<u8>);

#[cfg(feature = "uniffi")]
uniffi::custom_newtype!(EncryptedLabels, Vec<u8>);

impl EncryptedLabels {
    /// Encrypt the Labels struct using the given passphrase.
    pub fn encrypt(labels: &Labels, passphrase: &str) -> Result<Self, EncryptionError> {
        let labels = labels.export()?;

        let encrypted = {
            let encryptor =
                age::Encryptor::with_user_passphrase(Secret::new(passphrase.to_owned()));

            let mut encrypted = vec![];
            let mut writer = encryptor.wrap_output(&mut encrypted)?;

            writer.write_all(labels.as_bytes())?;
            writer.finish()?;

            encrypted
        };

        Ok(Self(encrypted))
    }

    /// Create a new EncryptedLabels struct from a hex encoded string.
    pub fn from_hex(hex: &str) -> Result<Self, EncryptionError> {
        let encrypted = hex::decode(hex)?;
        Ok(Self(encrypted))
    }

    /// Create a new EncryptedLabels struct from a file.
    pub fn read_from_file(path: impl AsRef<Path>) -> Result<Self, EncryptionError> {
        let path = path.as_ref();
        let encrypted = std::fs::read(path)?;

        Ok(Self(encrypted))
    }

    /// Get the encrypted bytes of the EncryptedLabels struct.
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    /// Decrypt the EncryptedLabels struct using the given passphrase.
    pub fn decrypt(&self, passphrase: &str) -> Result<Labels, EncryptionError> {
        let encrypted = &self.0;

        let decrypted = {
            let decryptor = match age::Decryptor::new(&encrypted[..])? {
                age::Decryptor::Passphrase(d) => d,
                _ => unreachable!(),
            };

            let mut decrypted = vec![];
            let mut reader = decryptor.decrypt(&Secret::new(passphrase.to_owned()), None)?;
            reader.read_to_end(&mut decrypted)?;

            decrypted
        };

        let labels_string = String::from_utf8(decrypted)?;
        let labels = Labels::try_from_str(&labels_string)?;

        Ok(labels)
    }

    /// Export the EncryptedLabels struct to a hex encoded string.
    pub fn to_hex(&self) -> Result<String, EncryptionError> {
        let encrypted = &self.0;
        let hex_encoded = hex::encode(encrypted);

        Ok(hex_encoded)
    }

    /// Export the EncryptedLabels struct to a file.
    pub fn write_to_file(&self, path: impl AsRef<Path>) -> Result<(), EncryptionError> {
        let path = path.as_ref();
        let encrypted = &self.0;

        let mut file = std::fs::File::create(path)?;
        file.write_all(encrypted)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{encryption::EncryptedLabels, Labels};

    #[test]
    fn test_encryption() {
        let labels = Labels::try_from_file("tests/data/labels.jsonl").unwrap();

        let encrypted = EncryptedLabels::encrypt(&labels, "passphrase").unwrap();
        let decrypted = encrypted.decrypt("passphrase").unwrap();

        assert_eq!(labels, decrypted);
    }
}
