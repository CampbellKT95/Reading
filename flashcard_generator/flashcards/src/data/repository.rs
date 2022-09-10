use redis::{RedisResult, Connection, Iter};

extern crate redis;

pub fn add_card(connection: &mut redis::Connection) -> i32 {
    // let test = connection.lpush("flashcards", "second test");
    let result: RedisResult<i32> = redis::cmd("LPUSH").arg("flashcards").arg("test flashcard value").query(connection);

    // should simply return # of entries (1)
    return result.unwrap();
}
//  -> Iter<String>
pub fn read_all_cards(connection: &mut redis::Connection) -> Iter<String> {

    let all_cards: redis::Iter<String> = redis::cmd("LRANGE").arg("flashcards").arg(0).arg(-1).clone().iter(connection).unwrap();

    return all_cards;
}