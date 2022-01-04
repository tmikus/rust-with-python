use crate::classified_message::ClassifiedMessage;
use crate::error::Error;

pub trait Classifier {
  fn classify<'a>(&self, message: &'a str) -> Result<ClassifiedMessage<'a>, Error>;
}
