use nu_plugin::Plugin;
use rpm::FromRpm;

mod rpm;
pub struct RpmPlugin;

impl Plugin for RpmPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![Box::new(FromRpm)]
    }
}
