use tokenizers::Tokenizer;

/// Load tokenizer from local file
pub fn create_tokenizer() -> Tokenizer {
    Tokenizer::from_file("data/tokenizer.json")
        .expect("Failed to load tokenizer")
}

/// Convert text into token IDs
pub fn tokenize_text(text: &str) -> Vec<u32> {
    let tokenizer = create_tokenizer();

    let encoding = tokenizer
        .encode(text, true)
        .expect("Tokenization failed");

    encoding.get_ids().to_vec()
}