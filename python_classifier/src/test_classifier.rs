use base_classifier::classifier::Classifier;
use base_classifier::classifier_result::ClassifierResult;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyModule, PyString};

pub struct TestClassifier {
  py_file_path: String,
}

impl TestClassifier {
  pub fn new(py_file_path: String) -> TestClassifier {
    TestClassifier {
      py_file_path,
    }
  }
}

impl Classifier for TestClassifier {
  fn classify<'a>(&self, message: &'a str) -> ClassifierResult<'a> {
    let result = Python::with_gil(|py| {
      let py_classify: Py<PyAny> = PyModule::from_code(
        py,
        "def classify(message):
            return [\"from-python\"]
        ",
        "",
        ""
      ).unwrap().getattr("classify").unwrap().into();
      let result = py_classify.call(py, (message,), None).unwrap();
      result.extract::<Vec<String>>(py).unwrap()
    });
    ClassifierResult::new(message, result)
  }
}
