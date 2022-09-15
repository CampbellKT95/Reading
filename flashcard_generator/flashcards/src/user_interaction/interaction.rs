use std::io;
use chrono::prelude::*;
use crate::flashcards;
use crate::repository;

fn ask_input() {
    println!("[1] Create a new card");
    println!("[2] View all current cards");
    println!("[3] Edit a card");
    println!("[4] Delete a card");
    println!("[0] Exit");
}

pub fn handle_input() {

    loop {
        ask_input();

        let mut user_resp = String::new();
        io::stdin().read_line(&mut user_resp).expect("Failed to read user input");
    
        if user_resp.as_str().trim() == "1" {
            let new_card: flashcards::Flashcard = create_card();

            let card_added: i32 = repository::add_card(new_card);
            println!("{}", card_added);
    
        } else if user_resp.as_str().trim() == "2" {
            let all_cards: Vec<String> = repository::read_all_cards();
            for card in all_cards {
                println!("{:?}", card);
            }
    
        } else if user_resp.as_str().trim() == "3" {
            let card_position: String = choose_card();
            repository::edit_card(card_position).expect("Failed to edit card");
    
        } else if user_resp.as_str().trim() == "4" {
            let card_position: String = choose_card();
            repository::delete_one_card(card_position).expect("Failed to delete item");
            
        } else if user_resp.as_str().trim() == "0" {
            println!("\nTerminating...\n");
            break;
    
        } else {
            println!("\nCommand not recognized...\n");
        }
    }
}

pub fn create_card() -> flashcards::Flashcard {
    println!("\nCreating card...\n");

    println!("Word:");
    let mut user_word = String::new();
    io::stdin().read_line(&mut user_word).expect("Failed to read user input");

    println!("Furigana:");
    let mut user_furigana = String::new();
    io::stdin().read_line(&mut user_furigana).expect("Failed to read user input");

    println!("Definition:");
    let mut user_definition = String::new();
    io::stdin().read_line(&mut user_definition).expect("Failed to read user input");

    println!("Example:");
    let mut user_example = String::new();
    io::stdin().read_line(&mut user_example).expect("Failed to read user input");

    let new_card = flashcards::Flashcard {
        word: user_word,
        furigana: user_furigana,
        definition: user_definition,
        example: user_example,
        difficulty: 10,
        last_reviewed: Local::now().to_string()
    };

    return new_card;
}

// for cards to edit/delete
fn choose_card() -> String {
    println!("Choose the position of card would you like to interact with");

    let mut all_card_position: Vec<flashcards::CardPosition> = Vec::new();

    let all_cards: Vec<String> = repository::read_all_cards();
    let mut position_counter: i32 = 0;

    for card in all_cards {
        println!("\n{}\n", card);

        let parsed_card: Result<flashcards::Flashcard, serde_json::Error> = serde_json::from_str(&card);

        let position = flashcards::CardPosition {
            position: position_counter,
            word: parsed_card.unwrap().word
        };
        position_counter += 1;

        all_card_position.push(position);
    }

    for card in all_card_position {
        println!("[{}]: {}", card.position, card.word);
    }

    let mut user_choice: String = String::new();
    io::stdin().read_line(&mut user_choice).expect("Failed to read card choice");

    println!("{}", user_choice);
    let position = user_choice.as_str().trim().to_owned();

    return position;
}