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
