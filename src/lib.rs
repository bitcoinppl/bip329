//! A library for working with [BIP329 labels](https://github.com/bitcoin/bips/blob/master/bip-0329.mediawiki).
//!
//! - The main data structure is the [`Labels`](crate::Labels) struct, which is a list of [`Label`](crate::Label) structs.
//! - The [`Label`](crate::Label) enum containing all the different types of labels.
//! - The [`Labels`](crate::Labels) struct can be imported/exported to/from a JSONL file.
//! - Supports encryption and decryption using the [`encryption`](crate::encryption) module.
//! - Supports the [`uniffi`](https://github.com/mozilla/uniffi-rs) feature, for easy integration with other languages.
//!
//! ### Example Import:
//! ```rust
//! use bip329::Labels;
//!
//! let labels = Labels::try_from_file("tests/data/labels.jsonl").unwrap();
//! ```
//!
//! ### Example Export:
//! ```rust
//! use bip329::Labels;
//!
//! // Create a Labels struct
//! let labels = Labels::try_from_file("tests/data/labels.jsonl").unwrap();
//!
//! // Create a JSONL string
//! let jsonl = labels.export().unwrap();
//! ```
//!
/// ### Example encryption (requires the `encryption` feature):
/// ```rust
#[cfg_attr(
    not(feature = "encryption"),
    doc = "# // This example requires the `encryption` feature"
)]
///
/// # #[cfg(feature = "encryption")]
/// # {
/// use bip329::{Labels, encryption::EncryptedLabels};
/// let labels = Labels::try_from_file("tests/data/labels.jsonl").unwrap();
/// let encrypted = EncryptedLabels::encrypt(&labels, "passphrase").unwrap();
///
/// let encrypted = EncryptedLabels::read_from_file("tests/data/encrypted_labels.age").unwrap();
/// let decrypted = encrypted.decrypt("passphrase").unwrap();
/// assert_eq!(labels, decrypted);
/// # }
/// ```
pub mod error;

#[cfg(feature = "encryption")]
pub mod encryption;

pub mod from;
mod label;
mod serde_util;

use bitcoin::{address::NetworkUnchecked, Address};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// A list of labels.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Labels(Vec<Label>);

/// The main data structure for BIP329 labels.
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

/// An enum representing all possible [`Label::ref_`]
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LabelRef {
    Txid(bitcoin::Txid),
    Address(bitcoin::Address<NetworkUnchecked>),
    PublicKey(String),
    Input(bitcoin::OutPoint),
    Output(bitcoin::OutPoint),
    Xpub(String),
}

impl Display for LabelRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            LabelRef::Txid(txid) => write!(f, "{txid}"),
            LabelRef::Address(address) => write!(f, "{}", address.clone().assume_checked()),
            LabelRef::PublicKey(pk) => write!(f, "{}", pk),
            LabelRef::Input(outpoint) => write!(f, "{}", outpoint),
            LabelRef::Output(outpoint) => write!(f, "{}", outpoint),
            LabelRef::Xpub(xpub) => write!(f, "{}", xpub),
        }
    }
}

/// A transaction label.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionRecord {
    #[serde(rename = "ref")]
    pub ref_: bitcoin::Txid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}

/// An address label.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AddressRecord {
    #[serde(rename = "ref")]
    pub ref_: Address<NetworkUnchecked>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// A public key label.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PublicKeyRecord {
    #[serde(rename = "ref")]
    pub ref_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// An input label.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InputRecord {
    #[serde(rename = "ref")]
    pub ref_: bitcoin::OutPoint,
    pub label: Option<String>,
}

/// An output label.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct OutputRecord {
    #[serde(rename = "ref")]
    pub ref_: bitcoin::OutPoint,

    pub label: Option<String>,

    #[serde(
        default = "default_true",
        deserialize_with = "serde_util::deserialize_string_or_bool"
    )]
    pub spendable: bool,
}

/// An extended public key label.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExtendedPublicKeyRecord {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub label: Option<String>,
}

impl OutputRecord {
    /// Defaults to being spendable if no spendable field is present
    pub fn spendable(&self) -> bool {
        self.spendable
    }
}

fn default_true() -> bool {
    true
}
