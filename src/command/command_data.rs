use crate::command::ctx::CommandContext;
use crate::json_objects::command::Command;

pub(crate) struct CommandDefinition<'a> {
    pub(crate) run: Box<&'a dyn Fn(&CommandContext)>,
    pub(crate) cooldown_ms: Option<u128>,
    pub(crate) on_denied: Option<Box<&'a dyn Fn(&CommandContext)>>,
    pub(crate) on_cooldown: Option<Box<&'a dyn Fn(&CommandContext)>>,
    pub(crate) case_sensitive: bool,
    pub(crate) command: Command
}
