pub fn train() {
    println!("Training started...");
    println!("Loading documents...");
    println!("Creating QA samples...");
    println!("Training model...");
    println!("Saving model...");

    std::fs::write("model.bin", "dummy model")
        .expect("Failed to save model");

    println!("Training complete!");
}