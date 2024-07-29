# BIP329

<p>
    <a href="https://crates.io/crates/bip329"><img alt="Crate Info" src="https://img.shields.io/crates/v/bip329.svg"/></a>
    <a href="https://github.com/bitcoinppl/bip329/blob/master/LICENSE"><img alt="Apache-2.0 Licensed" src="https://img.shields.io/badge/Apache--2.0-blue.svg"/></a>
    <a href="https://github.com/bitcoinppl/bip329/actions?query=workflow%3ACI"><img alt="CI Status" src="https://github.com/bitcoinppl/bip329/workflows/CI/badge.svg"></a>
    <a href="https://docs.rs/bip329"><img alt="Docs" src="https://img.shields.io/badge/docs.rs-green"/></a>
</p>

<!-- cargo-rdme start -->

A library for working with [BIP329 labels](https://github.com/bitcoin/bips/blob/master/bip-00329.mediawiki).

- The main data structure is the [`Labels`](https://docs.rs/bip329/latest/bip329/struct.Labels.html) struct, which is a list of [`Label`](https://docs.rs/bip329/latest/bip329/enum.Label.html) structs.
- The [`Label`](https://docs.rs/bip329/latest/bip329/enum.Label.html) enum containing all the different types of labels.
- The [`Labels`](https://docs.rs/bip329/latest/bip329/struct.Labels.html) struct can be imported/exported to/from a JSONL file.
- Supports encryption and decryption using the [`encryption`](https://docs.rs/bip329/latest/bip329/encryption/) module.
- Supports the [`uniffi`](https://github.com/mozilla/uniffi-rs) feature, for easy integration with other languages.

#### Example Import:
```rust
use bip329::Labels;

let labels = Labels::try_from_file("tests/data/labels.jsonl").unwrap();
```

#### Example Export:
```rust
use bip329::Labels;

// Create a Labels struct
let labels = Labels::try_from_file("tests/data/labels.jsonl").unwrap();

// Create a JSONL string
let jsonl = labels.export().unwrap();
```

#### Example encryption:
```rust
use bip329::{Labels, encryption::EncryptedLabels};

let labels = Labels::try_from_file("tests/data/labels.jsonl").unwrap();
let encrypted = EncryptedLabels::encrypt(&labels, "passphrase").unwrap();

let encrypted = EncryptedLabels::read_from_file("tests/data/encrypted_labels.age").unwrap();
let decrypted = encrypted.decrypt("passphrase").unwrap();
assert_eq!(labels, decrypted);
```

<!-- cargo-rdme end -->
