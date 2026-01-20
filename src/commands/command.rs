use crate::utils::cmnd_parser;

pub struct UserInput {
    pub raw: String,
}
impl UserInput {
    pub fn new() -> Self {
        Self { raw: String::new() }
    }
    pub fn args(&self) -> Vec<String>{
        cmnd_parser::handle(&self)
    }
}
