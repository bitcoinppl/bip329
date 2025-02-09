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
//! ### Example encryption:
//! ```rust
//! use bip329::{Labels, encryption::EncryptedLabels};
//!
//! let labels = Labels::try_from_file("tests/data/labels.jsonl").unwrap();
//! let encrypted = EncryptedLabels::encrypt(&labels, "passphrase").unwrap();
//!
//! let encrypted = EncryptedLabels::read_from_file("tests/data/encrypted_labels.age").unwrap();
//! let decrypted = encrypted.decrypt("passphrase").unwrap();
//! assert_eq!(labels, decrypted);
//! ```
//!
pub mod error;

#[cfg(feature = "encryption")]
pub mod encryption;

mod label;
mod serde_util;

use std::{num::ParseIntError, str::FromStr};

use bitcoin::{address::NetworkUnchecked, Address};
use serde::{
    de::{Error, Visitor},
    Deserialize, Serialize,
};

/// A list of labels.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Labels(Vec<Label>);

#[cfg(feature = "uniffi")]
uniffi::custom_newtype!(Labels, Vec<Label>);

/// The main data structure for BIP329 labels.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "type")]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
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

/// A transaction label.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
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
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct AddressRecord {
    #[serde(rename = "ref")]
    pub ref_: Address<NetworkUnchecked>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// A public key label.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct PublicKeyRecord {
    #[serde(rename = "ref")]
    pub ref_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// An input label.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct InputRecord {
    #[serde(rename = "ref")]
    pub ref_: InOutId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// An output label.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct OutputRecord {
    #[serde(rename = "ref")]
    pub ref_: InOutId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "serde_util::deserialize_string_or_bool"
    )]
    pub spendable: Option<bool>,
}

/// An extended public key label.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct ExtendedPublicKeyRecord {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub label: Option<String>,
}

impl OutputRecord {
    /// Defaults to being spendable if no spendable field is present
    pub fn spendable(&self) -> bool {
        self.spendable.unwrap_or(true)
    }
}

/// The ID for an input or output, which is a tuple of the transaction ID and the index of the input or output.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct InOutId {
    pub txid: bitcoin::Txid,
    pub index: u32,
}

/// The ID for an input or output, which is a tuple of the transaction ID and the index of the input or output.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Error))]
pub enum InOutIdError {
    #[error("Invalid InOutId format")]
    InvalidFormat,
    #[error("Invalid Txid {0:?}")]
    InvalidTxid(bitcoin::hex::HexToArrayError),
    #[error("Invalid index: {0:?}")]
    InvalidIndex(ParseIntError),
}

impl Serialize for InOutId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&format_args!("{}:{}", self.txid, self.index))
    }
}

impl FromStr for InOutId {
    type Err = InOutIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, ':');

        let txid = parts.next().ok_or(InOutIdError::InvalidFormat)?;
        let index = parts.next().ok_or(InOutIdError::InvalidFormat)?;

        let txid = bitcoin::Txid::from_str(txid).map_err(InOutIdError::InvalidTxid)?;
        let index = index.parse().map_err(InOutIdError::InvalidIndex)?;

        Ok(InOutId { txid, index })
    }
}

impl<'de> Deserialize<'de> for InOutId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct InOutIdVisitor;

        impl Visitor<'_> for InOutIdVisitor {
            type Value = InOutId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string in format 'txid:index'")
            }

            fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
                InOutId::from_str(value).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(InOutIdVisitor)
    }
}

#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();
