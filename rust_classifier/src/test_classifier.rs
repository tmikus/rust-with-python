use base_classifier::classifier::Classifier;
use base_classifier::classified_message::ClassifiedMessage;
use base_classifier::error::Error;

pub struct TestClassifier {
}

impl TestClassifier {
  pub fn new() -> TestClassifier {
    TestClassifier {}
  }
}

impl Classifier for TestClassifier {
  fn classify<'a>(&self, message: &'a str) -> Result<ClassifiedMessage<'a>, Error> {
    Ok(ClassifiedMessage::new(message, vec!["from-rust".to_owned()]))
  }
}
