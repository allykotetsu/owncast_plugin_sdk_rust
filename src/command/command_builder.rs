use crate::command::command_definition::CommandDefinition;
use crate::command::command_context::CommandContext;
use crate::json_objects::command::Command;

/// A struct for building a chat command.
pub struct CommandBuilder<'a> {
    name_: String,
    run_: Box<&'a dyn Fn(&CommandContext)>,
    cooldown_ms_: Option<u128>,
    on_denied_: Option<Box<&'a dyn Fn(&CommandContext)>>,
    description_: Option<String>,
    usage_: Option<String>,
    aliases_: Option<Vec<String>>,
    mod_only_: Option<bool>,
    on_cooldown_: Option<Box<&'a dyn Fn(&CommandContext)>>
}

impl<'a> CommandBuilder<'a> {
    /// Create a new Command, must have a name and a function for what happens when the command is run.
    pub fn new(name: &str, run: &'a fn(&CommandContext) -> ()) -> Self {
        Self {
            name_: name.to_string(),
            run_: Box::new(run),
            cooldown_ms_: None,
            on_denied_: None,
            description_: None,
            usage_: None,
            aliases_: None,
            mod_only_: None,
            on_cooldown_: None
        }
    }

    /// If the command has a cooldown, then how long is it.
    pub fn with_cooldown(mut self, cooldown: u128) -> Self {
        self.cooldown_ms_ = Some(cooldown);
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

    pub(crate) fn build(self, prefix: String, case_sensitive: bool) -> CommandDefinition<'a> {
        CommandDefinition {
            run: self.run_,
            on_denied: self.on_denied_,
            on_cooldown: self.on_cooldown_,
            command: Command {
                name: self.name_,
                prefix,
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