#[path = "data/connection.rs"] mod connection;
#[path = "data/repository.rs"] mod repository;
#[path = "user_interaction/interaction.rs"] mod interaction;
#[path = "models/flashcard.rs"] mod flashcards;

fn main() {
    // let mut connection = connection::connect();
    // repository::add_card(&mut connection);
    // repository::read_all_cards(&mut connection);
    interaction::handle_input();
}
