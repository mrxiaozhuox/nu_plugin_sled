
use commands::{open::SledOpen, save::SledSave};
use nu_plugin::{serve_plugin, MsgPackSerializer, Plugin};

mod commands;
mod value;

pub struct SledPlugin;
impl Plugin for SledPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![Box::new(SledOpen), Box::new(SledSave)]
    }
}

fn main() {
    serve_plugin(&SledPlugin, MsgPackSerializer);
}
