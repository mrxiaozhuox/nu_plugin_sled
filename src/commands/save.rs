use std::path::PathBuf;

use nu_plugin::{PluginCommand, SimplePluginCommand};
use nu_protocol::{LabeledError, Type, Value};

use crate::value::value_to_json;

pub struct SledSave;
impl SimplePluginCommand for SledSave {
    type Plugin = crate::SledPlugin;

    fn name(&self) -> &str {
        "sled-save"
    }

    fn description(&self) -> &str {
        "Save a record to sled db (encode by rmp-serde)"
    }

    fn examples(&self) -> Vec<nu_protocol::Example> {
        vec![nu_protocol::Example {
            example: "sled-open mydb | update name \"sled\" | sled-save mydb",
            description: "update database content",
            result: None,
        }]
    }

    fn signature(&self) -> nu_protocol::Signature {
        nu_protocol::Signature::build(PluginCommand::name(self))
            .required(
                "path",
                nu_protocol::SyntaxShape::String,
                "sled db path (a directory)",
            )
            .named("tree", nu_protocol::SyntaxShape::String, "save into tree", None)
            .input_output_type(Type::Record(Box::new([])), Type::Nothing)
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        input: &Value,
    ) -> Result<Value, nu_protocol::LabeledError> {
        let path = call.req::<String>(0)?;
        let path = PathBuf::from(&engine.get_current_dir()?).join(path);
        if let Value::Record { val, .. } = input {
            let input_value = val.clone();
            match sled::open(&path) {
                Ok(db) => {
                    let tree_flag = call.get_flag_value("tree");
                    if let Some(Value::String { val, .. }) = tree_flag {
                        let tree = db.open_tree(val.as_bytes());
                        if let Err(e) = &tree {
                            return Err(LabeledError::new("db error").with_label(
                                format!("failed to open tree: {:?}", e),
                                call.head,
                            ));
                        }
                        let tree = tree.unwrap();
                        for (key, value) in input_value.iter() {
                            let value = value_to_json(value);
                            let data = rmp_serde::encode::to_vec(&value);
                            match data {
                                Ok(v) => {
                                    let _ = tree.insert(key.as_str(), v);
                                },
                                _ => {},
                            }
                        }
                    } else {
                        for (key, value) in input_value.iter() {
                            let value = value_to_json(value);
                            let data = rmp_serde::encode::to_vec(&value);
                            match data {
                                Ok(v) => {
                                    let _ = db.insert(key.as_str(), v);
                                },
                                _ => {},
                            }
                        }
                    }
                    Ok(Value::nothing(call.head))
                },
                Err(e) => Err(LabeledError::new("db error")
                    .with_label(format!("failed to connect sled db: {:?}", e), call.head)),
            }
        } else {
            return Err(
                LabeledError::new("input error").with_label("data must be a record", call.head)
            )
        }
    }
}
