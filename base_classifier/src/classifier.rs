use crate::classifier_result::ClassifierResult;

pub trait Classifier {
  fn classify<'a>(&self, message: &'a str) -> ClassifierResult<'a>;
}
