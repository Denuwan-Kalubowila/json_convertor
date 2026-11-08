
use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn converts_json_file_to_jsonl_file() {
    let directory = tempdir().unwrap();

    let input_path = directory.path().join("input.json");
    let output_path = directory.path().join("output.jsonl");

    let input = r#"
    [
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"}
    ]
    "#;

    fs::write(&input_path, input).unwrap();

    let output = Command::new(
        env!("CARGO_BIN_EXE_json_convertor")
    )
    .arg(&input_path)
    .arg(&output_path)
    .output()
    .expect("Failed to execute CLI");

    // Verify the process succeeded.
    assert!(
        output.status.success(),
        "CLI failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Read the generated JSONL file.
    let actual = fs::read_to_string(&output_path).unwrap();

    // Verify the output.
    let expected = concat!(
        "{\"id\":1,\"name\":\"Alice\"}\n",
        "{\"id\":2,\"name\":\"Bob\"}\n"
    );

    assert_eq!(actual, expected);
}
