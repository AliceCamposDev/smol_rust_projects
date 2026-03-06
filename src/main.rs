mod parts_of_speech_mod;
mod nlp_search;

use crate::nlp_search::nlp_search::nlp_search_fn;
use crate::parts_of_speech_mod::parts_of_speech::parts_of_speech_fn;

use std::io::{self, Write};

fn main() {
    loop {
        println!("\n=== NLP TOOL ===");
        println!("1 - Parts of Speech");
        println!("2 - NLP Search");
        println!("0 - Exit");
        print!("Select an option: ");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("\nRunning POS tagger...\n");
                parts_of_speech_fn();
            }
            "2" => {
                println!("\nRunning NLP search...\n");
                nlp_search_fn();
            }
            "0" => {
                println!("Bye!");
                break;
            }
            _ => {
                println!("Invalid option");
            }
        }
    }
}