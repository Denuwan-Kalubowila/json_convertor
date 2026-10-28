use serde_json::Value;
use std::error::Error;
use std::io::{Error as IoError, ErrorKind};

pub fn parse_json(content: &str) -> Result<Value, serde_json::Error> {
    serde_json::from_str(content)
}

pub fn format_json(value: &Value) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(value)
}

pub fn json_to_jsonl(content: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let value: Value = serde_json::from_str(content)?;

    let records = match &value {
        Value::Array(records) => records,
        _ => {
            return Err(
                IoError::new(ErrorKind::InvalidInput, "Input JSON must be an array").into(),
            );
        }
    };

    let lines = records
        .iter()
        .map(serde_json::to_string)
        .collect::<Result<Vec<String>, serde_json::Error>>()?;

    Ok(lines)
}

pub fn jsonl_to_json(lines: &[String]) -> Result<String, Box<dyn std::error::Error>> {
    let mut records: Vec<Value> = Vec::new();

    for (index, line) in lines.iter().enumerate() {
        let line_num = index + 1;
        let trimed = line.trim();

        if trimed.is_empty() {
            continue;
        }

        let record: Value = serde_json::from_str(trimed).map_err(|error| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid json on line {line_num}: {error}"),
            )
        })?;

        records.push(record);
    }
    let output = serde_json::to_string_pretty(&records)?;
    Ok(output)
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_json_array_to_jsonl() {
        let input = r#"
        [
            {"id": 1, "name": "Alice"},
            {"id": 2, "name": "Bob"}
        ]
        "#;

        let result = json_to_jsonl(input).unwrap();

        assert_eq!(
            result,
            vec![
                r#"{"id":1,"name":"Alice"}"#,
                r#"{"id":2,"name":"Bob"}"#
            ]
        );
    }

    #[test]
    fn rejects_invalid_json() {
        let input = r#"{"id",1"#;

        let result = json_to_jsonl(input);

        assert!(result.is_err());
    }

     #[test]
    fn rejects_json_object_instead_of_array() {
        let input = r#"{"id": 1, "name": "Alice"}"#;

        let result = json_to_jsonl(input);

        assert!(result.is_err());
    }

    #[test]
    fn converts_jsonl_array_to_json_array() {
        let lines = vec![
            r#"{"id":1,"name":"Alice"}"#.to_string(),
            r#"{"id":2,"name":"Bob"}"#.to_string(),
        ];

        let result = jsonl_to_json(&lines).unwrap();
        let actual:serde_json::Value = serde_json::from_str(&result).unwrap();

        let expected = serde_json::json!([
            {"id": 1, "name": "Alice"},
            {"id": 2, "name": "Bob"}
        ]);

        assert_eq!(actual,expected);
    }

    #[test]
    fn ignores_empty_jsonl_lines() {
        let lines = vec![
            r#"{"id":1}"#.to_string(),
            "".to_string(),
            "   ".to_string(),
            r#"{"id":2}"#.to_string(),
        ];

        let result = jsonl_to_json(&lines).unwrap();
        let actual: serde_json::Value =
            serde_json::from_str(&result).unwrap();

        let expected = serde_json::json!([
            {"id": 1},
            {"id": 2}
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn reports_invalid_jsonl_line_number() {
        let lines = vec![
            r#"{"id":1}"#.to_string(),
            r#"{"id": "#.to_string(),
        ];

        let error = jsonl_to_json(&lines).unwrap_err();

        assert!(error.to_string().contains("line 2"));
    }

}