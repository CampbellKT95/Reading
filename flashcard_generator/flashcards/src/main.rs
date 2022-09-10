#[path = "data/connection.rs"] mod connection;

fn main() {
    // https://docs.rs/redis/latest/redis/
    println!("Hello, world!");
    connection::connect();
    println!("I think we connected?");
}
