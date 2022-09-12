use std::io;
use redis::{Iter};
use crate::flashcards;
use crate::repository;
use crate::connection;

fn ask_input() {
    println!("[1] Create a new card");
    println!("[2] View all current cards");
    println!("[3] Edit a card");
    println!("[4] Delete a card");
}

pub fn handle_input() {

    loop {
        ask_input();

        let mut user_resp = String::new();
        io::stdin().read_line(&mut user_resp).expect("Failed to read user input");
    
        let mut connection = connection::connect();
    
        if user_resp.as_str().trim() == "1" {
            let new_card: flashcards::Flashcard = create_card();

            let card_added: i32 = repository::add_card(&mut connection, new_card);
            println!("{}", card_added);
    
        } else if user_resp.as_str().trim() == "2" {
            let all_cards: Iter<String> = repository::read_all_cards(&mut connection);
            for card in all_cards {
                println!("{:?}", card);
            }
    
        } else if user_resp.as_str().trim() == "3" {
            repository::edit_card(&mut connection);
    
        } else if user_resp.as_str().trim() == "4" {
            repository::delete_one_card(&mut connection);
            
        } else if user_resp.as_str().trim() == "0" {
            println!("\nTerminating...\n");
            break;
    
        } else {
            println!("\nCommand not recognized...\n");
        }
    }
}

fn create_card() -> flashcards::Flashcard {
    let new_card = flashcards::Flashcard {
        word: "Hello".to_string(),
        furigana: "H-E-L-L-O".to_string(),
        definition: "Greeting".to_string(),
        example: "Hello, World".to_string(),
        difficulty: 1,
        last_reviewed: "Today".to_string(),
    };

    return new_card;
}