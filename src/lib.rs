pub mod error;
pub mod label;
mod serde_util;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Labels(Vec<Label>);

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "type")]
/// Labels are the main data structure for BIP329 labels.
/// They are a list of Labels, each of which is a type of label.
/// The type of label is determined by the `type` field.
pub enum Label {
    #[serde(rename = "tx")]
    Transaction(TransactionRecord),
    #[serde(rename = "addr")]
    Address(AddressRecord),
    #[serde(rename = "pubkey")]
    PublicKey(PublicKeyRecord),
    #[serde(rename = "input")]
    Input(InputRecord),
    #[serde(rename = "output")]
    Output(OutputRecord),
    #[serde(rename = "xpub")]
    ExtendedPublicKey(ExtendedPublicKeyRecord),
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// A transaction label.
pub struct TransactionRecord {
    #[serde(rename = "ref")]
    ref_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// An address label.
pub struct AddressRecord {
    #[serde(rename = "ref")]
    ref_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// A public key label.
pub struct PublicKeyRecord {
    #[serde(rename = "ref")]
    ref_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InputRecord {
    #[serde(rename = "ref")]
    ref_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct OutputRecord {
    #[serde(rename = "ref")]
    ref_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "serde_util::deserialize_string_or_bool"
    )]
    spendable: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExtendedPublicKeyRecord {
    #[serde(rename = "ref")]
    ref_: String,
    label: Option<String>,
}

impl OutputRecord {
    /// Defaults to being spendable if no spendable field is present
    pub fn spendable(&self) -> bool {
        self.spendable.unwrap_or(true)
    }
}
