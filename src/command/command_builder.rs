use crate::command::command_data::CommandData;
use crate::command::ctx::Ctx;
use crate::json_objects::command::Command;

/// A struct for building a chat command.
pub struct CommandBuilder<'a> {
    name: String,
    run: Box<&'a dyn Fn(Ctx)>,
    cooldown: Option<u128>,
    on_denied: Option<Box<&'a dyn Fn(Ctx)>>,
    description: Option<String>,
    usage: Option<String>,
    aliases: Option<Vec<String>>,
    mod_only: Option<bool>
}

impl<'a> CommandBuilder<'a> {
    /// Create a new Command, must have a name and a function for what happens when the command is run.
    pub fn new<F: Fn(Ctx) -> () + 'static>(name: &str, run: &'a F) -> Self {
        Self {
            name: name.to_string(),
            run: Box::new(run),
            cooldown: None,
            on_denied: None,
            description: None,
            usage: None,
            aliases: None,
            mod_only: None,
        }
    }

    /// If the command has a cooldown, then how long is it.
    pub fn with_cooldown(mut self, cooldown: u128) -> Self {
        self.cooldown = Some(cooldown);
        self
    }

    /// Aliases this command uses.
    pub fn with_aliases(mut self, aliases: &[&str]) -> Self {
        let mut v = vec![];
        for alias in aliases {
            v.push(alias.to_string());
        }
        self.aliases = Some(v);
        self
    }

    pub(crate) fn build(self, prefix: String) -> CommandData<'a> {
        CommandData {
            run: self.run,
            cooldown: self.cooldown,
            on_denied: self.on_denied,
            command: Command {
                name: self.name,
                prefix,
                description: self.description,
                usage: self.usage,
                aliases: self.aliases,
                mod_only: self.mod_only,
            }
        }
    }
}