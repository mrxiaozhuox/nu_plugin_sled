use std::path::PathBuf;

use nu_plugin::{PluginCommand, SimplePluginCommand};
use nu_protocol::{LabeledError, Record, Type, Value};

use serde_json::Value as JsonValue;

pub struct SledOpen;
impl SimplePluginCommand for SledOpen {
    type Plugin = crate::SledPlugin;

    fn name(&self) -> &str {
        "sled-open"
    }

    fn description(&self) -> &str {
        "Open a sled database by path (rmp-serde value only)"
    }

    fn examples(&self) -> Vec<nu_protocol::Example> {
        let record = nu_protocol::record! {
            "id" => Value::test_int(1),
            "name" => Value::test_string("hello"),
            "items" => Value::test_list(vec! [
                Value::test_string("apple"),
                Value::test_string("banana"),
            ])
        };
        vec![nu_protocol::Example {
            example: "sled-open mydb",
            description: "open a sled db `mydb` and load all data",
            result: Some(Value::test_record(record)),
        }]
    }

    fn signature(&self) -> nu_protocol::Signature {
        nu_protocol::Signature::build(PluginCommand::name(self))
            .required(
                "path",
                nu_protocol::SyntaxShape::String,
                "db path (a directory)",
            )
            .switch("raw", "load raw data (binary data)", None)
            .named(
                "tree",
                nu_protocol::SyntaxShape::String,
                "load from tree",
                None,
            )
            .named(
                "prefix",
                nu_protocol::SyntaxShape::String,
                "load data by prefix",
                None,
            )
            .input_output_type(Type::Nothing, Type::Record(Box::new([])))
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, nu_protocol::LabeledError> {
        let path = call.req::<String>(0)?;
        let path = PathBuf::from(&engine.get_current_dir()?).join(path);

        if !path.is_dir() {
            return Err(LabeledError::new("db error")
                .with_label(format!("directory not found: {:?}", path), call.head));
        }

        match sled::open(&path) {
            Ok(db) => {
                let tree_flag = call.get_flag_value("tree");
                if let Some(Value::String { val, .. }) = tree_flag {
                    let tree = db.open_tree(val.as_bytes());
                    match tree {
                        Ok(tree) => {
                            let mut iter = tree.iter();
                            if let Some(Value::String { val, .. }) = call.get_flag_value("prefix") {
                                iter = tree.scan_prefix(val.as_bytes());
                            }
                            read_from_iter(iter, call)
                        }
                        Err(e) => Err(LabeledError::new("db error")
                            .with_label(format!("failed to open tree: {:?}", e), call.head)),
                    }
                } else {
                    let mut iter = db.iter();
                    if let Some(Value::String { val, .. }) = call.get_flag_value("prefix") {
                        iter = db.scan_prefix(val.as_bytes());
                    }
                    read_from_iter(iter, call)
                }
            }
            Err(e) => Err(LabeledError::new("db error")
                .with_label(format!("failed to connect sled db: {:?}", e), call.head)),
        }
    }
}

fn read_from_iter<I>(iter: I, call: &nu_plugin::EvaluatedCall) -> Result<Value, LabeledError>
where
    I: Iterator<Item = Result<(sled::IVec, sled::IVec), sled::Error>>,
{
    let mut record = Record::new();

    let raw = call.has_flag("raw").unwrap_or(false);

    for (k, v) in iter.flatten() {
        let key = String::from_utf8_lossy(&k).to_string();
        if raw {
            record.insert(key, Value::binary(v.to_vec(), call.head));
            continue;
        }
        match rmp_serde::decode::from_slice::<JsonValue>(&v) {
            Ok(decoded_value) => {
                let nu_value = crate::value::json_to_value(&decoded_value, call.head);
                record.insert(key, nu_value);
            }
            Err(err) => {
                return Err(LabeledError::new("decode error").with_label(
                    format!("failed to decode value for key {}: {:?}", key, err),
                    call.head,
                ));
            }
        }
    }

    Ok(Value::record(record, call.head))
}
