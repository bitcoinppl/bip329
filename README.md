# bip329

A library for working with (BIP329 labels)[https://github.com/bitcoin/bips/blob/master/bip-00329.mediawiki].

This library provides a way to work with BIP329 labels in a Rust program.

The main data structure is the `Labels` struct, which is a list of `Label` structs.

The `Label` enum is a discriminated union of all the different types of labels.

The `Labels` struct can be exported to a JSONL file.

The `Labels` struct can be imported from a JSONL file.

Example Import:

```rust
use bip329::Labels;

let labels = Labels::try_from_file("tests/data/labels.jsonl").unwrap();
```

Example Export:

```rust
use bip329::Labels;

// Create a Labels struct
let labels = Labels::try_from_file("tests/data/labels.jsonl").unwrap();

// Create a JSONL string
let jsonl = labels.export().unwrap();


License: Apache-2.0
```
