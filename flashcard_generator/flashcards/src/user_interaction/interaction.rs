use std::io;
use chrono::prelude::*;
use crate::flashcards;
use crate::repository;

fn ask_input() {
    println!("[1] Create a new card");
    println!("[2] View all current cards");
    println!("[3] Edit a card");
    println!("[4] Delete a card");
    println!("[5] Clear lists");
    println!("[0] Exit\n");
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
            
            // parse cards from json into their objects, then print them            
            for card in all_cards {
                let parsed_card: flashcards::Flashcard = serde_json::from_str(&card).unwrap();
                println!("\n | Word: {} | Furigana: {} | Definition: {} | Example: {} | Difficulty: {} | Last Review: {}\n", parsed_card.word, parsed_card.furigana, parsed_card.definition, parsed_card.example, parsed_card.difficulty, parsed_card.last_reviewed);
                println!("-------------------------------------------------------------------\n");
            }
    
        } else if user_resp.as_str().trim() == "3" {
            let card_position: String = choose_card();
            repository::edit_card(card_position).expect("Failed to edit card");
    
        } else if user_resp.as_str().trim() == "4" {
            let card_position: String = choose_card();
            repository::delete_one_card(card_position).expect("Failed to delete item");

        } else if user_resp.as_str().trim() == "5" {
            repository::clear_lists();
            
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
    println!("\nHow would you like to choose a card?\n");

    println!("[1] Type out word");
    println!("[2] View all cards & select position");

    let mut method_choice: String = String::new();
    io::stdin().read_line(&mut method_choice).expect("Failed to read user method choice");

    // the position being searched for
    let mut user_choice_position: String = String::new();

    if method_choice.to_string().trim() == "1" {
        println!("\nPlease type the word you wish to choose\n");

        let mut desired_word: String = String::new();
        io::stdin().read_line(&mut desired_word).expect("Failed to read user's word");

        let word_position: i32 = repository::find_index(&desired_word);
        user_choice_position = word_position.to_string();

    } else if method_choice.to_string().trim() == "2" {
        let mut all_card_position: Vec<flashcards::CardPosition> = Vec::new();

        let all_cards: Vec<String> = repository::read_all_cards();
        let mut position_counter: i32 = 0;
    
        for card in all_cards {
            println!("\n{}\n", card);
    
            let parsed_card: flashcards::Flashcard = serde_json::from_str(&card).unwrap();
    
            let position = flashcards::CardPosition {
                position: position_counter,
                word: parsed_card.word
            };
            position_counter += 1;
    
            all_card_position.push(position);
        }
    
        for card in all_card_position {
            println!("[{}]: {}", card.position, card.word);
        }

        println!("\n Please type the position of the word you wish to select\n");
        io::stdin().read_line(&mut user_choice_position).expect("Failed to read card choice");

    } else {
        println!("Command not recognized, returning to main menu");
    }

    let position = user_choice_position.as_str().trim().to_owned();
    return position;
}