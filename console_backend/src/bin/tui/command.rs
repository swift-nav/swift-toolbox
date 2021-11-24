use anyhow::Result;

enum Command {
    Global(GlobalCommands),
    SettingsView(SettingsViewCommands),
}

impl Command {
    pub fn parse(&self) -> Result<Self> {
        todo!()
    }
    pub fn exec(&self) {}
}

enum SettingsViewCommands {
    WriteSetting { value: String },
}

impl SettingsViewCommands {
    fn name(&self) -> &'static str {
        todo!()
    }
}

enum GlobalCommands {
    Quit,
    Help,
    WriteSetting {
        group: String,
        name: String,
        value: String,
    },
}

fn collect_args(cmd: &str) -> Vec<&str> {
    cmd.split_ascii_whitespace().skip(1).collect()
}
