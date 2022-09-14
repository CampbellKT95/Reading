#[path = "data/connection.rs"] mod connection;
#[path = "data/repository.rs"] mod repository;
#[path = "user_interaction/interaction.rs"] mod interaction;
#[path = "models/flashcard.rs"] mod flashcards;

fn main() {
    interaction::handle_input();
}
