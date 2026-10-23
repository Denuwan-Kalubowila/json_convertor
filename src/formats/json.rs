use serde_json::Value;

pub fn parse_json(content: &str) -> Result<Value, serde_json::Error> {
    serde_json::from_str(content)
}

pub fn format_json(value: &Value) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(value)
}
