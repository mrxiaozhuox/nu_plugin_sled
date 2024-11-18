use nu_protocol::{Record, Span, Value};

use serde_json::Value as JsonValue;

pub fn value_to_json(value: &Value) -> JsonValue {
    match value {
        Value::Int { val, .. } => JsonValue::from(*val),
        Value::Float { val, .. } => JsonValue::from(*val),
        Value::String { val, .. } => JsonValue::from(val.clone()),
        Value::Bool { val, .. } => JsonValue::from(*val),
        Value::Date { val, .. } => JsonValue::from(val.timestamp()),
        Value::Duration { val, .. } => JsonValue::from(*val),
        Value::List { vals, .. } => {
            let json_vals: Vec<JsonValue> = vals.iter().map(|v| value_to_json(v)).collect();
            JsonValue::Array(json_vals)
        }
        Value::Record { val, .. } => {
            let mut map = serde_json::Map::new();
            for (k, v) in val.iter() {
                map.insert(k.clone(), value_to_json(v));
            }
            JsonValue::Object(map)
        }
        v => JsonValue::String(v.to_debug_string()),
    }
}

pub fn json_to_value(json: &JsonValue, span: Span) -> Value {
    match json {
        JsonValue::Null => Value::nothing(span),
        JsonValue::Bool(b) => Value::bool(*b, span),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::int(i, span)
            } else if let Some(f) = n.as_f64() {
                Value::float(f, span)
            } else {
                Value::nothing(span)
            }
        }
        JsonValue::String(s) => Value::string(s, span),
        JsonValue::Array(arr) => {
            let vals: Vec<Value> = arr.iter().map(|v| json_to_value(v, span)).collect();
            Value::list(vals, span)
        }
        JsonValue::Object(obj) => {
            let mut record = Record::new();
            for (k, v) in obj {
                record.insert(k.to_string(), json_to_value(v, span));
            }
            Value::record(record, span)
        }
    }
}
