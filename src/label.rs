use crate::{
    error::{ExportError, ParseError},
    Label, Labels,
};
use std::{
    fs::File,
    io::{BufRead as _, BufReader},
    ops::{Deref, DerefMut},
    path::Path,
};

impl Labels {
    /// Create a new Labels struct.
    pub fn new(labels: Vec<Label>) -> Self {
        Labels(labels)
    }

    /// Create a new Labels struct from a string.
    pub fn try_from_str(labels: &str) -> Result<Self, ParseError> {
        let labels = labels
            .lines()
            .map(serde_json::from_str)
            .collect::<Result<Vec<Label>, _>>()?;

        Ok(Labels(labels))
    }

    /// Create a new Labels struct from a file.
    pub fn try_from_file(path: impl AsRef<Path>) -> Result<Self, ParseError> {
        let file = File::open(path.as_ref())?;
        let buffer_reader = BufReader::new(file);

        let labels = buffer_reader
            .lines()
            .map(|line| {
                let line = &line.map_err(ParseError::FileReadError)?;
                let label: Label = serde_json::from_str(line).map_err(ParseError::ParseError)?;
                Ok::<Label, ParseError>(label)
            })
            .collect::<Result<Vec<Label>, _>>()?;

        Ok(Self::new(labels))
    }

    /// Export the Labels struct to a string.
    pub fn export(&self) -> Result<String, ExportError> {
        let contents = self
            .0
            .iter()
            .map(serde_json::to_string)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(contents.join("\n"))
    }

    /// Export the Labels struct to a file.
    pub fn export_to_file(&self, path: &str) -> Result<(), ExportError> {
        let contents = self.export()?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    /// Export the Labels struct to a writer.
    pub fn export_to_writer<W: std::io::Write>(&self, mut writer: W) -> Result<(), ExportError> {
        self.0.iter().try_for_each(|label: &Label| {
            let label = serde_json::to_string(label)?;
            writer.write_all(label.as_bytes())?;
            writer.write_all(b"\n")?;
            Ok(())
        })
    }

    /// Get the inner Vec of the Labels struct.
    pub fn into_vec(self) -> Vec<Label> {
        self.0
    }
}

impl Label {
    pub fn try_from_str(label: &str) -> Result<Self, ParseError> {
        let label: Label = serde_json::from_str(label)?;
        Ok(label)
    }
}

impl Deref for Labels {
    type Target = Vec<Label>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Labels {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use serde_json::from_str;

    use crate::*;

    #[test]
    fn test_deserialization() {
        let test_vector = r#"{"type": "tx", "ref": "f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd", "label": "Transaction", "origin": "wpkh([d34db33f/84'/0'/0'])"}
{"type": "addr", "ref": "bc1q34aq5drpuwy3wgl9lhup9892qp6svr8ldzyy7c", "label": "Address"}
{"type": "pubkey", "ref": "0283409659355b6d1cc3c32decd5d561abaac86c37a353b52895a5e6c196d6f448", "label": "Public Key"}
{"type": "input", "ref": "f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd:0", "label": "Input"}
{"type": "output", "ref": "f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd:1", "label": "Output", "spendable": false}
{"type": "xpub", "ref": "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8", "label": "Extended Public Key"}
{"type": "tx", "ref": "f546156d9044844e02b181026a1a407abfca62e7ea1159f87bbeaa77b4286c74", "label": "Account #1 Transaction", "origin": "wpkh([d34db33f/84'/0'/1'])"}"#;

        let records: Vec<Label> = test_vector
            .lines()
            .filter_map(|line| from_str(line).ok())
            .collect();

        assert_eq!(records.len(), 7);

        // Test Transaction
        if let Label::Transaction(TransactionRecord {
            ref_,
            label,
            origin,
        }) = &records[0]
        {
            assert_eq!(
                ref_,
                "f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd"
            );
            assert_eq!(label, &Some("Transaction".to_string()));
            assert_eq!(origin, &Some("wpkh([d34db33f/84'/0'/0'])".to_string()));
        } else {
            panic!("Expected Transaction");
        }

        // Test Address
        if let Label::Address(AddressRecord { ref_, label }) = &records[1] {
            assert_eq!(ref_, "bc1q34aq5drpuwy3wgl9lhup9892qp6svr8ldzyy7c");
            assert_eq!(label, &Some("Address".to_string()));
        } else {
            panic!("Expected Address");
        }

        // Test PublicKey
        if let Label::PublicKey(PublicKeyRecord { ref_, label }) = &records[2] {
            assert_eq!(
                ref_,
                "0283409659355b6d1cc3c32decd5d561abaac86c37a353b52895a5e6c196d6f448"
            );
            assert_eq!(label, &Some("Public Key".to_string()));
        } else {
            panic!("Expected PublicKey");
        }

        // Test Input
        if let Label::Input(InputRecord { ref_, label }) = &records[3] {
            assert_eq!(
                ref_,
                "f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd:0"
            );
            assert_eq!(label, &Some("Input".to_string()));
        } else {
            panic!("Expected Input");
        }

        // Test Output
        if let Label::Output(OutputRecord {
            ref_,
            label,
            spendable,
        }) = &records[4]
        {
            assert_eq!(
                ref_,
                "f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd:1"
            );
            assert_eq!(label, &Some("Output".to_string()));
            assert_eq!(spendable, &Some(false));
        } else {
            panic!("Expected Output");
        }

        // Test ExtendedPublicKey
        if let Label::ExtendedPublicKey(ExtendedPublicKeyRecord { ref_, label }) = &records[5] {
            assert_eq!(ref_, "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8");
            assert_eq!(label, &Some("Extended Public Key".to_string()));
        } else {
            panic!("Expected ExtendedPublicKey");
        }

        // Test second Transaction
        if let Label::Transaction(TransactionRecord {
            ref_,
            label,
            origin,
        }) = &records[6]
        {
            assert_eq!(
                ref_,
                "f546156d9044844e02b181026a1a407abfca62e7ea1159f87bbeaa77b4286c74"
            );
            assert_eq!(label, &Some("Account #1 Transaction".to_string()));
            assert_eq!(origin, &Some("wpkh([d34db33f/84'/0'/1'])".to_string()));
        } else {
            panic!("Expected Transaction");
        }

        let spendable_output_ommited = r#"{"type": "output", "ref": "f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd:1", "label": "Output" }"#;

        let label = Label::try_from_str(spendable_output_ommited).unwrap();

        if let Label::Output(
            record @ OutputRecord {
                ref_,
                label,
                spendable,
            },
        ) = &label
        {
            assert_eq!(
                ref_,
                "f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd:1"
            );
            assert_eq!(*label, Some("Output".to_string()));
            assert_eq!(*spendable, None);
            assert!(record.spendable());
        };
    }

    #[test]
    fn test_export_to_writer() {
        let mut buffer = Vec::new();

        let labels = Labels::try_from_file("tests/data/labels.jsonl").unwrap();
        labels.export_to_writer(&mut buffer).unwrap();

        let jsonl_string = std::str::from_utf8(&buffer).unwrap().trim();
        let expected = std::fs::read_to_string("tests/data/labels.jsonl").unwrap();

        assert_eq!(jsonl_string, expected);
    }
}
