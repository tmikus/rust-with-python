use std::borrow::Cow;
use base_classifier::classifier::Classifier;
use base_classifier::classifier_result::ClassifierResult;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyModule, PyString};

const CLASSIFIER: &'static [u8] = include_bytes!("classifier.py");

pub struct TestClassifier {
  classifier_code: Cow<'static, str>,
  py_file_path: String,
}

impl TestClassifier {
  pub fn new(py_file_path: String) -> TestClassifier {
    TestClassifier {
      classifier_code: String::from_utf8_lossy(CLASSIFIER),
      py_file_path,
    }
  }
}

impl Classifier for TestClassifier {
  fn classify<'a>(&self, message: &'a str) -> ClassifierResult<'a> {
    let result = Python::with_gil(|py| {
      let py_classify: Py<PyAny> = PyModule::from_code(
        py,
        &self.classifier_code,
        "classifier.py",
        "python_classifier"
      ).unwrap().getattr("classify").unwrap().into();
      let result = py_classify.call(py, (message,), None).unwrap();
      result.extract::<Vec<String>>(py).unwrap()
    });
    ClassifierResult::new(message, result)
  }
}
