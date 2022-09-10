#[path = "data/connection.rs"] mod connection;
#[path = "data/repository.rs"] mod repository;

fn main() {
    let mut connection = connection::connect();
    // repository::add_card(&mut connection);
    repository::read_all_cards(&mut connection);
}
