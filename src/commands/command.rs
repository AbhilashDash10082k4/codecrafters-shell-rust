pub struct UserInput {
   pub raw: String,
}
impl UserInput {
   pub fn new(raw: String) -> Self {
      Self { raw }
   }
}
