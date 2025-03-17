use crate::{
    error::{ExportError, ParseError},
    Label, LabelRef, Labels, TransactionRecord,
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
        Self(labels)
    }

    /// Create a new Labels struct from a string.
    pub fn try_from_str(labels: &str) -> Result<Self, ParseError> {
        let labels = labels
            .trim()
            .lines()
            .map(serde_json::from_str)
            .collect::<Result<Vec<Label>, _>>()?;

        Ok(Self(labels))
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

    /// Get the full transaction label record
    pub fn transaction_label_record(&self) -> Option<&TransactionRecord> {
        self.0.iter().find_map(|label: &Label| {
            if let Label::Transaction(record) = label {
                return Some(record);
            }

            None
        })
    }

    /// Get the transaction label
    pub fn transaction_label(&self) -> Option<&str> {
        let record = self.transaction_label_record()?;
        let label = record.label.as_ref()?.as_str();

        if label.is_empty() {
            return None;
        }

        Some(label)
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
    pub fn export_to_file(&self, path: impl AsRef<Path>) -> Result<(), ExportError> {
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
    #[must_use]
    pub fn into_vec(self) -> Vec<Label> {
        self.0
    }

    /// Get an iterator over the Labels struct.
    pub fn iter(&self) -> impl Iterator<Item = &Label> {
        self.0.iter()
    }
}

impl Label {
    /// Create a new Label struct from a string.
    pub fn try_from_str(label: &str) -> Result<Self, ParseError> {
        let label: Self = serde_json::from_str(label)?;
        Ok(label)
    }

    /// Returns the ref as a LabelRef
    pub fn ref_(&self) -> LabelRef {
        match self {
            Label::Transaction(record) => LabelRef::Txid(record.ref_),
            Label::Address(record) => LabelRef::Address(record.ref_.clone()),
            Label::PublicKey(record) => LabelRef::PublicKey(record.ref_.clone()),
            Label::Input(record) => LabelRef::Input(record.ref_),
            Label::Output(record) => LabelRef::Output(record.ref_),
            Label::ExtendedPublicKey(record) => LabelRef::Xpub(record.ref_.clone()),
        }
    }

    /// return the `label` value
    pub fn label(&self) -> Option<String> {
        match self {
            Label::Transaction(record) => record.label.clone(),
            Label::Address(record) => record.label.clone(),
            Label::PublicKey(record) => record.label.clone(),
            Label::Input(record) => record.label.clone(),
            Label::Output(record) => record.label.clone(),
            Label::ExtendedPublicKey(record) => record.label.clone(),
        }
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

impl From<Vec<Label>> for Labels {
    fn from(value: Vec<Label>) -> Self {
        Self(value)
    }
}

impl From<Labels> for Vec<Label> {
    fn from(value: Labels) -> Self {
        value.0
    }
}

impl Default for Labels {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use bitcoin::Txid;
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
                &Txid::from_str("f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd")
                    .unwrap()
            );
            assert_eq!(label, &Some("Transaction".to_string()));
            assert_eq!(origin, &Some("wpkh([d34db33f/84'/0'/0'])".to_string()));
        } else {
            panic!("Expected Transaction");
        }

        // Test Address
        if let Label::Address(AddressRecord { ref_, label }) = &records[1] {
            assert_eq!(
                ref_,
                &Address::from_str("bc1q34aq5drpuwy3wgl9lhup9892qp6svr8ldzyy7c").unwrap()
            );
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
                &bitcoin::OutPoint::from_str(
                    "f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd:0"
                )
                .unwrap()
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
                &bitcoin::OutPoint::from_str(
                    "f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd:1"
                )
                .unwrap()
            );
            assert_eq!(label, &Some("Output".to_string()));
            assert!(!*spendable);
        } else {
            panic!("Expected Output");
        }

        // Test ExtendedPublicKey
        if let Label::ExtendedPublicKey(ExtendedPublicKeyRecord { ref_, label }) = &records[5] {
            assert_eq!(
                ref_,
                "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8"
            );
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
                &Txid::from_str("f546156d9044844e02b181026a1a407abfca62e7ea1159f87bbeaa77b4286c74")
                    .unwrap()
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
                &bitcoin::OutPoint::from_str(
                    "f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd:1"
                )
                .unwrap()
            );
            assert_eq!(*label, Some("Output".to_string()));
            assert!(*spendable);
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

        let jsonl = Labels::try_from_str(jsonl_string).unwrap();
        let expected = Labels::try_from_str(&expected).unwrap();

        assert_eq!(jsonl, expected);
    }
}
