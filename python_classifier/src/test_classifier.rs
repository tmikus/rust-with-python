use std::borrow::Cow;
use pyo3::AsPyPointer;
use base_classifier::classifier::Classifier;
use base_classifier::classified_message::ClassifiedMessage;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyModule, PyString};
use base_classifier::error::Error;

const CLASSIFIER: &'static [u8] = include_bytes!("classifier.py");

pub struct TestClassifier {
  classify_function: *mut pyo3::ffi::PyObject,
}

impl TestClassifier {
  pub fn new() -> Result<TestClassifier, Error> {
    let classifier_code = String::from_utf8_lossy(CLASSIFIER);
    Python::with_gil(|py| {
      let module = match PyModule::from_code(
        py,
        &classifier_code,
        "classifier.py",
        "python_classifier"
      ) {
        Ok(module) => module,
        Err(err) => {
          eprintln!("An error occurred when initialising classifier module: {}", err);
          return Err(Error::UnknownError);
        }
      };
      let classify_function = match module.getattr("classify") {
        Ok(value) if value.is_callable() => value.as_ptr(),
        Ok(_) => {
          eprintln!("An error occurred when initialising classifier module: 'classify' function is not callable");
          return Err(Error::UnknownError);
        },
        Err(err) => {
          eprintln!("An error occurred when initialising classifier module: {}", err);
          return Err(Error::UnknownError);
        },
      };
      Ok(TestClassifier { classify_function })
    })
  }

  fn parse_classifier_result<'a>(message: &'a str, result: &PyAny) -> Result<ClassifiedMessage<'a>, Error> {
    match result.extract::<Vec<String>>() {
      Ok(classifications) => Ok(ClassifiedMessage::new(message, classifications)),
      Err(err) => {
        eprintln!("An error occurred when parsing classifier result: {}", err);
        Err(Error::UnknownError)
      },
    }
  }
}

impl Classifier for TestClassifier {
  fn classify<'a>(&self, message: &'a str) -> Result<ClassifiedMessage<'a>, Error> {
    Python::with_gil(|py| {
      let classify_function: &PyAny = unsafe { py.from_borrowed_ptr(self.classify_function) };
      match classify_function.call((message,), None) {
        Ok(result) => TestClassifier::parse_classifier_result(message, result),
        Err(err) => {
          eprintln!("An error occurred when classifying message: {}", err);
          Err(Error::UnknownError)
        }
      }
    })
  }
}
