use crate::command::command_context::CommandContext;
use crate::json_objects::command::Command;

pub(crate) struct CommandDefinition {
    pub(crate) run: fn(&CommandContext),
    pub(crate) command: Command
}