use base_classifier::classifier::Classifier;
use base_classifier::error::Error;

fn get_classifier() -> Result<impl Classifier, Error> {
    // rust_classifier::test_classifier::TestClassifier::new()
    python_classifier::test_classifier::TestClassifier::new()
}

fn main() {
    let classifier = match get_classifier() {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Init error: {:?}", err);
            return;
        }
    };
    match classifier.classify("Test message") {
        Ok(result) => {
            println!("Result {:?}", result);
        },
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
}
