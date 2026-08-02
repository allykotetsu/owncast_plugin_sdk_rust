use crate::command::command_definition::CommandDefinition;
use crate::command::command_context::CommandContext;
use crate::plugin::event_function::{EventFunctionVoid};
use crate::json_objects::command_info::CommandInfo;

/// A struct for building a chat command.
pub struct CommandBuilder {
    name_: String,
    run_: EventFunctionVoid<CommandContext>,
    cooldown_ms_: Option<i64>,
    description_: Option<String>,
    usage_: Option<String>,
    aliases_: Option<Vec<String>>,
    mod_only_: Option<bool>,
}

impl CommandBuilder {
    /// Create a new Command, must have a name and a function for what happens when the command is run.
    pub fn new(name: &str, run: EventFunctionVoid<CommandContext>) -> Self {
        Self {
            name_: name.to_string(),
            run_: run,
            cooldown_ms_: None,
            description_: None,
            usage_: None,
            aliases_: None,
            mod_only_: None,
        }
    }

    /// If the command has a cooldown, then how long is it.
    pub fn with_cooldown(mut self, cooldown: i64) -> Self {
        self.cooldown_ms_ = Some(cooldown);
        self
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description_ = Some(desc.to_string());
        self
    }

    pub fn with_usage(mut self, usage: &str) -> Self {
        self.usage_ = Some(usage.to_string());
        self
    }

    /// Aliases this command uses.
    pub fn with_aliases(mut self, aliases: &[&str]) -> Self {
        let mut v = vec![];
        for alias in aliases {
            v.push(alias.to_string());
        }
        self.aliases_ = Some(v);
        self
    }

    pub fn mod_only(mut self) -> Self {
        self.mod_only_ = Some(true);
        self
    }

    pub(crate) fn build(self, prefix: String, case_sensitive: bool) -> CommandDefinition {
        CommandDefinition {
            run: self.run_,
            command: CommandInfo {
                name: self.name_,
                prefix: Some(prefix),
                description: self.description_,
                usage: self.usage_,
                aliases: self.aliases_,
                mod_only: self.mod_only_,
                case_sensitive: Some(case_sensitive),
                cooldown_ms: self.cooldown_ms_
            }
        }
    }
}