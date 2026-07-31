use crate::command::command_context::CommandContext;
use crate::event_function::EventFunctionVoid;
use crate::json_objects::command_info::CommandInfo;

pub(crate) struct CommandDefinition {
    pub(crate) run: EventFunctionVoid<CommandContext>,
    pub(crate) command: CommandInfo
}