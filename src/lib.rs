pub mod error;
pub mod label;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Labels(Vec<Label>);

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "type")]
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
pub struct TransactionRecord {
    #[serde(rename = "ref")]
    ref_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AddressRecord {
    #[serde(rename = "ref")]
    ref_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

    #[serde(skip_serializing_if = "Option::is_none")]
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
