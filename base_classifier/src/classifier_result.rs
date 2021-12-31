#[derive(Debug)]
pub struct ClassifierResult<'a> {
  classifications: Vec<String>,
  message: &'a str,
}

impl<'a> ClassifierResult<'a> {
  pub fn new(message: &'a str, classifications: Vec<String>) -> ClassifierResult {
    ClassifierResult {
      classifications,
      message,
    }
  }
}
