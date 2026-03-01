pub fn answer(question: &str) {
    println!("Question received:");
    println!("{}", question);

    let q = question.to_lowercase();

    // Example rule-based answers (we will upgrade later)
    if q.contains("graduation") {
        println!("Answer: April 14");
    } else if q.contains("term 1") {
        println!("Answer: Term 1 starts in January.");
    } else {
        println!("Answer: Information found in calendar documents.");
    }
}