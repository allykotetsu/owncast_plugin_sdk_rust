use crate::json_objects::user::User;

pub(crate) struct Ctx {
    pub(crate) msg: String,
    pub(crate) user: User,
    pub(crate) command: String,
    pub(crate) args: Vec<String>,
    pub(crate) arg_string: String
}

impl Ctx {
    pub fn reply(&self, _msg: &str) {
        todo!()
    }

    pub fn reply_privately(&self, _msg: &str) {
        todo!()
    }
}