mod parts_of_speech_mod;
mod nlp_search;
mod stt;
mod file_handling;
mod translator;
mod neural_network;
mod load_vault;

use crate::neural_network::neural_network::neural_network_fn;
use crate::translator::translator::translator_fn;
use crate::file_handling::file_handling::file_handling_fn;
use crate::nlp_search::nlp_search::nlp_search_fn;
use crate::parts_of_speech_mod::parts_of_speech::parts_of_speech_fn;
use crate::stt::stt::stt_fn;
use crate::load_vault::load_vault::load_vault_fn;


use std::io::{self, Write};

fn main() {
    loop {
        println!("\n=== NLP TOOL ===");
        println!("1 - Parts of Speech");
        println!("2 - NLP Search");
        println!("3 - STT");
        println!("4 - File Handling");
        println!("5 - Translator");
        println!("6 - Neural Network");
        println!("7 - Load vault");
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
            "3" => {
                println!("\nRunning STT...\n");
                stt_fn();
            }
            "4" => {
                println!("\nRunning File Handling...\n");
                file_handling_fn();
            }
            "5" => {
                println!("\nRunning Translator...\n");
                translator_fn();
            }
            "6" => {
                println!("\nRunning Neural Network...\n");
                neural_network_fn();
            }
            "7" => {
                println!("\n");
                load_vault_fn();
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