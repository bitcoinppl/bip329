use bip329::Labels;

#[test]
fn test_loop_back() {
    use pretty_assertions::assert_eq;

    let labels_1 = Labels::try_from_file("tests/data/labels.jsonl").unwrap();
    let export_json = labels_1.export().unwrap();

    let labels_2 = Labels::try_from_str(&export_json).unwrap();

    assert_eq!(labels_1, labels_2);
}

#[test]
fn loop_back_test_vector() {
    use pretty_assertions::assert_eq;

    let labels_1 = Labels::try_from_file("tests/data/test_vector.jsonl").unwrap();
    let export_json = labels_1.export().unwrap();

    let labels_2 = Labels::try_from_str(&export_json).unwrap();

    assert_eq!(labels_1, labels_2);
}
