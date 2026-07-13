use crate::command::command_context::CommandContext;
use crate::json_objects::command::Command;

pub(crate) struct CommandDefinition<'a> {
    pub(crate) run: Box<&'a dyn Fn(&CommandContext)>,
    pub(crate) on_denied: Option<Box<&'a dyn Fn(&CommandContext)>>,
    pub(crate) on_cooldown: Option<Box<&'a dyn Fn(&CommandContext)>>,
    pub(crate) command: Command
}