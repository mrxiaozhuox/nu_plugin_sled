use std::path::PathBuf;

use nu_plugin::{PluginCommand, SimplePluginCommand};
use nu_protocol::{LabeledError, Type, Value};

pub struct SledTrees;
impl SimplePluginCommand for SledTrees {
    type Plugin = crate::SledPlugin;

    fn name(&self) -> &str {
        "sled-trees"
    }

    fn description(&self) -> &str {
        "list all trees in the sled db"
    }

    fn signature(&self) -> nu_protocol::Signature {
        nu_protocol::Signature::build(PluginCommand::name(self))
            .required(
                "path",
                nu_protocol::SyntaxShape::String,
                "sled db path (a directory)",
            )
            .input_output_type(Type::Nothing, Type::List(Box::new(Type::String)))
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
                let trees = db.tree_names();
                let list = trees
                    .iter()
                    .filter_map(|v| {
                        if let Ok(name) = String::from_utf8(v.to_vec()) {
                            Some(Value::string(name, call.head))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                Ok(Value::list(list, call.head))
            }
            Err(e) => Err(LabeledError::new("db error")
                .with_label(format!("failed to connect sled db: {:?}", e), call.head)),
        }
    }
}
