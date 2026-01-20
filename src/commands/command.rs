pub struct UserInput {
    pub raw: String,
}
impl UserInput {
    pub fn new () -> Self {
        Self {raw: String::new()}
    }
}