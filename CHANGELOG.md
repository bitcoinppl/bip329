# Changelog

## Unreleased

- Update dependencies

### Breaking Changes

- Use bitcoin types as the `ref` field for `Transaction`, `Address`, `Input`, and `Output` label
- Remove support for `uniffi`
- `spendable` field is now a boolean instead of an option boolean
- `spendable` field will always be serialized as a boolean,
  - it won't be `null` if it's `false`
  - it won't be omitted if it's `true`

### Added

- `FromStr` for `InOutId`
- Convenient `From` impls for `Label`
- IntoIterator for `Labels`
- `iter` function for `Labels`

## [0.1.2] - 2023-07-28

- Update dependencies

## [0.1.1] - 2023-07-28

- Documentation formatting

## [0.1.0] - 2023-07-28

- Import and export BIP329 labels from and to JSONL files
- Basic encryption and decryption support
