use crate::command::ctx::Ctx;
use crate::json_objects::command::Command;

pub(crate) struct CommandData<'a> {
    pub(crate) run: Box<&'a dyn Fn(Ctx)>,
    pub(crate) cooldown: Option<u128>,
    pub(crate) on_denied: Option<Box<&'a dyn Fn(Ctx)>>,
    pub(crate) command: Command
}
