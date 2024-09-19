use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_rpm::RpmPlugin;

fn main() {
    serve_plugin(&RpmPlugin {}, MsgPackSerializer {})
}

#[cfg(test)]
mod tests {
    use nu_plugin_rpm::RpmPlugin;
    use nu_plugin::Plugin;
    use nu_protocol::ShellError;


    #[test]
    pub fn test_examples() -> Result<(), ShellError> {
        use nu_plugin_test_support::PluginTest;

        let p = &RpmPlugin{};

        for cmd in p.commands() {
            PluginTest::new("rpm", RpmPlugin.into())?.test_examples(&cmd.examples())?;
        }

        Ok(())
    }
}