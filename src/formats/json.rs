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
