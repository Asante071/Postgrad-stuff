mod training;
mod inference;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("cargo run -- train");
        println!("cargo run -- ask \"your question\"");
        return;
    }

    match args[1].as_str() {
        "train" => training::trainer::train(),

        "ask" => {
            if args.len() < 3 {
                println!("Please provide a question.");
                return;
            }
            inference::predict::answer(&args[2]);
        }

        _ => println!("Unknown command"),
    }
}