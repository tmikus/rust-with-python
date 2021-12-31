use base_classifier::classifier::Classifier;

fn get_classifier() -> impl Classifier {
    // rust_classifier::test_classifier::TestClassifier::new()
    python_classifier::test_classifier::TestClassifier::new("asdf".to_owned())
}

fn main() {
    let classifier = get_classifier();
    let result = classifier.classify("Test message");
    println!("Result {:?}", result);
}
