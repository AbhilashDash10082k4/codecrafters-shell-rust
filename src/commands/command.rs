
pub struct UserInput {
    pub raw: String,
}
impl UserInput {
    pub fn new () -> Self {
        Self {raw: String::new()}
    }
    pub fn name_and_args (&self) -> (&str, &str) {
        let ip_cmd = self.raw.trim();
        match ip_cmd.find(' ') {
            Some(i) => {
                let echo_cmd = &ip_cmd[..i];
                let arg = ip_cmd[i+1..].trim();
                return (echo_cmd, arg);
            },
            None => (ip_cmd, "")
        }
    }
}