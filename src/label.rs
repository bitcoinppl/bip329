use crate::{
    error::{ExportError, ParseError},
    AddressRecord, ExtendedPublicKeyRecord, InputRecord, Label, LabelParseOptions, LabelRef,
    Labels, OutputRecord, OutputSpendableField, ParsedLabels, PublicKeyRecord,
    SilentPaymentsScanRecord, SpendableFieldValue, TransactionRecord,
};
use std::{
    collections::HashMap,
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

    /// Create labels from JSONL when normalized BIP329 records are enough
    ///
    /// Use this for normal imports where only the explicit `spendable` value is
    /// needed and the original JSON representation can be normalized
    pub fn try_from_str(labels: &str) -> Result<Self, ParseError> {
        Self::try_from_str_with_options(labels, LabelParseOptions::default())
    }

    /// Create labels from JSONL using custom parse options
    ///
    /// Known record types accept additional JSON fields by default. Set
    /// [`LabelParseOptions::ignore_unknown_types`] to skip records with
    /// unsupported `type` values.
    pub fn try_from_str_with_options(
        labels: &str,
        options: LabelParseOptions,
    ) -> Result<Self, ParseError> {
        let mut parsed_labels = Vec::new();

        for line in labels.trim().lines() {
            if let Some(label) = parse_label_line(line, options)? {
                parsed_labels.push(label);
            }
        }

        Ok(Self(parsed_labels))
    }

    /// Create labels while preserving output `spendable` field metadata
    ///
    /// Use this when callers need to distinguish omitted `spendable` fields from
    /// explicitly provided booleans or string booleans
    pub fn try_from_str_with_metadata(labels: &str) -> Result<ParsedLabels, ParseError> {
        Self::try_from_str_with_metadata_and_options(labels, LabelParseOptions::default())
    }

    /// Create labels with metadata using custom parse options
    ///
    /// Unknown `type` values are skipped when
    /// [`LabelParseOptions::ignore_unknown_types`] is enabled.
    pub fn try_from_str_with_metadata_and_options(
        labels: &str,
        options: LabelParseOptions,
    ) -> Result<ParsedLabels, ParseError> {
        let mut output_spendable = Vec::new();
        let mut parsed_labels = Vec::new();

        for line in labels.trim().lines() {
            let Some(line) = parse_label_line_with_metadata(line, options)? else {
                continue;
            };
            let (label, spendable) = line.into_label_and_spendable();
            parsed_labels.push(label);

            if let Some(spendable) = spendable {
                output_spendable.push(spendable);
            }
        }

        Ok(ParsedLabels {
            labels: Self(parsed_labels),
            output_spendable,
        })
    }

    /// Create a new Labels struct from a file.
    pub fn try_from_file(path: impl AsRef<Path>) -> Result<Self, ParseError> {
        Self::try_from_file_with_options(path, LabelParseOptions::default())
    }

    /// Create labels from a file using custom parse options.
    pub fn try_from_file_with_options(
        path: impl AsRef<Path>,
        options: LabelParseOptions,
    ) -> Result<Self, ParseError> {
        let file = File::open(path.as_ref())?;
        let buffer_reader = BufReader::new(file);

        let mut labels = Vec::new();

        for line in buffer_reader.lines() {
            let line = line.map_err(ParseError::FileReadError)?;
            if let Some(label) = parse_label_line(&line, options)? {
                labels.push(label);
            }
        }

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

    /// Get the inner Vec of the Labels struct converted to a HashMap
    pub fn into_map(self) -> HashMap<LabelRef, Label> {
        self.into_iter().map(|l| (l.ref_(), l)).collect()
    }

    /// Get the inner Vec of the Labels struct, with string keys
    pub fn into_string_map(self) -> HashMap<String, Label> {
        self.into_iter()
            .map(|l| (l.ref_().to_string(), l))
            .collect()
    }

    /// Get an iterator over the Labels struct.
    pub fn iter(&self) -> impl Iterator<Item = &Label> {
        self.0.iter()
    }
}

fn parse_label_line(line: &str, options: LabelParseOptions) -> Result<Option<Label>, ParseError> {
    if should_skip_unknown_label_type(line, options)? {
        return Ok(None);
    }

    let label = serde_json::from_str(line)?;
    Ok(Some(label))
}

fn parse_label_line_with_metadata(
    line: &str,
    options: LabelParseOptions,
) -> Result<Option<ParsedLabelLine>, ParseError> {
    if should_skip_unknown_label_type(line, options)? {
        return Ok(None);
    }

    let label = serde_json::from_str(line)?;
    Ok(Some(label))
}

fn should_skip_unknown_label_type(
    line: &str,
    options: LabelParseOptions,
) -> Result<bool, ParseError> {
    if !options.ignore_unknown_types {
        return Ok(false);
    }

    let label_type: LabelType = serde_json::from_str(line)?;
    Ok(!is_known_label_type(&label_type.type_))
}

fn is_known_label_type(label_type: &str) -> bool {
    matches!(
        label_type,
        "tx" | "addr" | "pubkey" | "input" | "output" | "xpub" | "spscan"
    )
}

#[derive(serde::Deserialize)]
struct LabelType {
    #[serde(rename = "type")]
    type_: String,
}

#[derive(serde::Deserialize)]
#[serde(tag = "type")]
enum ParsedLabelLine {
    #[serde(rename = "tx")]
    Transaction(TransactionRecord),
    #[serde(rename = "addr")]
    Address(AddressRecord),
    #[serde(rename = "pubkey")]
    PublicKey(PublicKeyRecord),
    #[serde(rename = "input")]
    Input(InputRecord),
    #[serde(rename = "output")]
    Output(ParsedOutputRecord),
    #[serde(rename = "xpub")]
    ExtendedPublicKey(ExtendedPublicKeyRecord),
    #[serde(rename = "spscan")]
    SilentPaymentsScan(SilentPaymentsScanRecord),
}

impl ParsedLabelLine {
    fn into_label_and_spendable(self) -> (Label, Option<OutputSpendableField>) {
        match self {
            Self::Transaction(record) => (Label::Transaction(record), None),
            Self::Address(record) => (Label::Address(record), None),
            Self::PublicKey(record) => (Label::PublicKey(record), None),
            Self::Input(record) => (Label::Input(record), None),
            Self::Output(record) => record.into_label_and_spendable(),
            Self::ExtendedPublicKey(record) => (Label::ExtendedPublicKey(record), None),
            Self::SilentPaymentsScan(record) => (Label::SilentPaymentsScan(record), None),
        }
    }
}

#[derive(serde::Deserialize)]
struct ParsedOutputRecord {
    #[serde(rename = "ref")]
    ref_: bitcoin::OutPoint,
    label: Option<String>,
    #[serde(default)]
    spendable: SpendableFieldValue,
}

impl ParsedOutputRecord {
    fn into_label_and_spendable(self) -> (Label, Option<OutputSpendableField>) {
        let label = Label::Output(OutputRecord {
            ref_: self.ref_,
            label: self.label,
            spendable: self.spendable.explicit_value(),
        });
        let output_spendable = OutputSpendableField {
            ref_: self.ref_,
            value: self.spendable,
        };

        (label, Some(output_spendable))
    }
}

impl Label {
    /// Create a new Label struct from a string.
    pub fn try_from_str(label: &str) -> Result<Self, ParseError> {
        let label: Self = serde_json::from_str(label)?;
        Ok(label)
    }

    /// return the `label` as a str
    pub fn label(&self) -> Option<&str> {
        match self {
            Label::Transaction(record) => record.label.as_deref(),
            Label::Address(record) => record.label.as_deref(),
            Label::PublicKey(record) => record.label.as_deref(),
            Label::Input(record) => record.label.as_deref(),
            Label::Output(record) => record.label.as_deref(),
            Label::ExtendedPublicKey(record) => record.label.as_deref(),
            Label::SilentPaymentsScan(record) => record.label.as_deref(),
        }
    }

    /// Get the reference of the label as a &str
    pub fn ref_(&self) -> LabelRef {
        match self {
            Label::Transaction(record) => LabelRef::Txid(record.ref_),
            Label::Address(record) => LabelRef::Address(record.ref_.clone()),
            Label::PublicKey(record) => LabelRef::PublicKey(record.ref_.clone()),
            Label::Input(record) => LabelRef::Input(record.ref_),
            Label::Output(record) => LabelRef::Output(record.ref_),
            Label::ExtendedPublicKey(record) => LabelRef::Xpub(record.ref_.clone()),
            Label::SilentPaymentsScan(record) => LabelRef::SilentPaymentsScan(record.ref_.clone()),
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
{"type": "spscan", "ref": "spscan1q5zs2pg9q5zs2pg9q5zs2pg9q5zs2pg9q5zs2pg9q5zs2pg9q5zsq9q6qjevn2kmdrnpuxt0v6h2kr2a2epkr0g6nk55ftf0xcxtddazgkrth3e", "label": "Silent Payments Scan Key Expression"}
{"type": "tx", "ref": "f546156d9044844e02b181026a1a407abfca62e7ea1159f87bbeaa77b4286c74", "label": "Account #1 Transaction", "origin": "wpkh([d34db33f/84'/0'/1'])"}"#;

        let records: Vec<Label> = test_vector
            .lines()
            .filter_map(|line| from_str(line).ok())
            .collect();

        assert_eq!(records.len(), 8);

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
            assert_eq!(*spendable, Some(false));
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

        // Test SilentPaymentsScan
        if let Label::SilentPaymentsScan(SilentPaymentsScanRecord { ref_, label }) = &records[6] {
            assert_eq!(
                ref_,
                "spscan1q5zs2pg9q5zs2pg9q5zs2pg9q5zs2pg9q5zs2pg9q5zs2pg9q5zsq9q6qjevn2kmdrnpuxt0v6h2kr2a2epkr0g6nk55ftf0xcxtddazgkrth3e"
            );
            assert_eq!(
                label,
                &Some("Silent Payments Scan Key Expression".to_string())
            );
        } else {
            panic!("Expected SilentPaymentsScan");
        }

        // Test second Transaction
        if let Label::Transaction(TransactionRecord {
            ref_,
            label,
            origin,
        }) = &records[7]
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
            assert_eq!(*spendable, None);
            assert!(record.spendable());
        };
    }

    #[test]
    fn test_output_spendable_metadata_omitted() {
        let jsonl = r#"{"type": "output", "ref": "f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd:1", "label": "Output" }"#;

        let labels = Labels::try_from_str_with_metadata(jsonl).unwrap();
        let Label::Output(record) = &labels.labels[0] else {
            panic!("Expected Output");
        };

        assert_eq!(record.spendable, None);
        assert_eq!(
            labels.output_spendable[0].value,
            SpendableFieldValue::Omitted
        );
    }

    #[test]
    fn test_output_spendable_metadata_boolean() {
        let jsonl = r#"{"type": "output", "ref": "f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd:1", "label": "Output", "spendable": false}"#;

        let labels = Labels::try_from_str_with_metadata(jsonl).unwrap();
        let Label::Output(record) = &labels.labels[0] else {
            panic!("Expected Output");
        };

        assert_eq!(record.spendable, Some(false));
        assert_eq!(
            labels.output_spendable[0].value,
            SpendableFieldValue::Boolean(false)
        );
    }

    #[test]
    fn test_output_spendable_metadata_string() {
        let jsonl = r#"{"type": "output", "ref": "f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd:1", "label": "Output", "spendable": "true"}"#;

        let labels = Labels::try_from_str_with_metadata(jsonl).unwrap();
        let Label::Output(record) = &labels.labels[0] else {
            panic!("Expected Output");
        };

        assert_eq!(record.spendable, Some(true));
        assert_eq!(
            labels.output_spendable[0].value,
            SpendableFieldValue::String(true)
        );
    }

    #[test]
    fn output_export_omits_absent_spendable() {
        let jsonl = r#"{"type": "output", "ref": "f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd:1", "label": "Output" }"#;

        let labels = Labels::try_from_str(jsonl).unwrap();
        let exported = labels.export().unwrap();

        assert!(exported.contains(r#""label":"Output""#));
        assert!(!exported.contains("spendable"));
    }

    #[test]
    fn spscan_label_helpers_return_record_data() {
        let scan_key = "spscan1q5zs2pg9q5zs2pg9q5zs2pg9q5zs2pg9q5zs2pg9q5zs2pg9q5zsq9q6qjevn2kmdrnpuxt0v6h2kr2a2epkr0g6nk55ftf0xcxtddazgkrth3e";
        let jsonl = format!(
            r#"{{"type": "spscan", "ref": "{scan_key}", "label": "Silent Payments Scan Key Expression"}}"#
        );

        let label = Label::try_from_str(&jsonl).unwrap();

        assert_eq!(label.label(), Some("Silent Payments Scan Key Expression"));
        assert_eq!(
            label.ref_(),
            LabelRef::SilentPaymentsScan(scan_key.to_string())
        );
    }

    #[test]
    fn known_records_ignore_additional_fields() {
        let jsonl = r#"{"type": "tx", "ref": "f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd", "label": "Transaction", "height": 1, "rate": {"USD": 105620.0}}"#;

        let labels = Labels::try_from_str(jsonl).unwrap();
        let Label::Transaction(record) = &labels[0] else {
            panic!("Expected Transaction");
        };

        assert_eq!(record.label.as_deref(), Some("Transaction"));
    }

    #[test]
    fn can_ignore_unknown_record_types() {
        let jsonl = r#"{"type": "tx", "ref": "f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd", "label": "Transaction"}
{"type": "future", "ref": {"not": "validated"}, "label": "Unknown"}
{"type": "addr", "ref": "bc1q34aq5drpuwy3wgl9lhup9892qp6svr8ldzyy7c", "label": "Address"}"#;

        assert!(Labels::try_from_str(jsonl).is_err());

        let labels = Labels::try_from_str_with_options(
            jsonl,
            LabelParseOptions::default().ignore_unknown_types(true),
        )
        .unwrap();

        assert_eq!(labels.len(), 2);
        assert!(matches!(labels[0], Label::Transaction(_)));
        assert!(matches!(labels[1], Label::Address(_)));
    }

    #[test]
    fn metadata_parser_can_ignore_unknown_record_types() {
        let jsonl = r#"{"type": "tx", "ref": "f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd", "label": "Transaction"}
{"type": "future", "ref": {"not": "validated"}, "label": "Unknown"}
{"type": "output", "ref": "f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd:1", "label": "Output", "spendable": "false"}"#;

        let labels = Labels::try_from_str_with_metadata_and_options(
            jsonl,
            LabelParseOptions::default().ignore_unknown_types(true),
        )
        .unwrap();

        assert_eq!(labels.labels.len(), 2);
        assert_eq!(labels.output_spendable.len(), 1);
        assert_eq!(
            labels.output_spendable[0].value,
            SpendableFieldValue::String(false)
        );
    }

    #[test]
    fn test_output_spendable_metadata_mixed_labels() {
        let jsonl = r#"{"type": "tx", "ref": "f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd", "label": "Transaction"}
{"type": "output", "ref": "f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd:1", "label": "Output", "spendable": "false"}
{"type": "addr", "ref": "bc1q34aq5drpuwy3wgl9lhup9892qp6svr8ldzyy7c", "label": "Address"}"#;

        let labels = Labels::try_from_str_with_metadata(jsonl).unwrap();

        assert_eq!(labels.labels.len(), 3);
        assert_eq!(labels.output_spendable.len(), 1);
        assert_eq!(
            labels.output_spendable[0].value,
            SpendableFieldValue::String(false)
        );

        let Label::Output(record) = &labels.labels[1] else {
            panic!("Expected Output");
        };
        assert_eq!(record.spendable, Some(false));
        assert!(!record.spendable());
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
