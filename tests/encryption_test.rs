use bip329::{encryption::EncryptedLabels, Labels};

#[test]
fn test_decryption() {
    let labels = Labels::try_from_file("tests/data/labels.jsonl").unwrap();
    let encrypted = EncryptedLabels::read_from_file("tests/data/encrypted_labels.age").unwrap();
    let decrypted = encrypted.decrypt("passphrase").unwrap();

    assert_eq!(labels, decrypted);
}

#[test]
fn test_loop_back_encryption() {
    use pretty_assertions::assert_eq;

    let labels_1 = Labels::try_from_file("tests/data/labels.jsonl").unwrap();
    let encrypted = EncryptedLabels::encrypt(&labels_1, "passphrase").unwrap();
    let decrypted = encrypted.decrypt("passphrase").unwrap();

    assert_eq!(labels_1, decrypted);
}
