#[derive(Debug)]
pub struct ClassifiedMessage<'a> {
  classifications: Vec<String>,
  message: &'a str,
}

impl<'a> ClassifiedMessage<'a> {
  pub fn new(message: &'a str, classifications: Vec<String>) -> ClassifiedMessage {
    ClassifiedMessage {
      classifications,
      message,
    }
  }
}
