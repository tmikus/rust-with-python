use base_classifier::classifier::Classifier;
use base_classifier::classifier_result::ClassifierResult;

pub struct TestClassifier {
}

impl TestClassifier {
  pub fn new() -> TestClassifier {
    TestClassifier {}
  }
}

impl Classifier for TestClassifier {
  fn classify<'a>(&self, message: &'a str) -> ClassifierResult<'a> {
    ClassifierResult::new(message, vec!["from-rust".to_owned()])
  }
}
