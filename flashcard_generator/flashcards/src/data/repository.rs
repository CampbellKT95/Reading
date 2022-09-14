use crate::flashcards;
use crate::interaction;
use crate::connection;
use redis::{RedisResult, Iter};
extern crate redis;

pub fn add_card(new_card: flashcards::Flashcard ) -> i32 {
    let mut connection = connection::connect();

    // convert newly created card to JSON
    let s_card = serde_json::to_string(&new_card).unwrap();

    let result: RedisResult<i32> = redis::cmd("LPUSH").arg("flashcards").arg(s_card).query(&mut connection);

    // should simply return # of entries (1)
    return result.unwrap();
}

pub fn read_all_cards() -> Iter<String> {
    let mut connection = connection::connect();

    let all_cards: redis::Iter<String> = redis::cmd("LRANGE").arg("flashcards").arg(0).arg(-1).clone().iter(&mut connection).unwrap();

    return all_cards;
}

// for edit & delete, find the position of the item first with LPOS, then LSET/LREM it
pub fn edit_card(card_position: String) -> Result<i32, redis::RedisError> {
        let mut connection = connection::connect();

        let updated_card: flashcards::Flashcard = interaction::create_card();
        let serialized_card: String = serde_json::to_string(&updated_card).unwrap();

        let result: RedisResult<i32> = redis::cmd("LSET").arg("flashcards").arg(card_position).arg(serialized_card).query(&mut connection);

        return result;
}

pub fn delete_one_card(card_position: String) -> Result<i32, redis::RedisError> {
    let mut connection = connection::connect();

    println!("PRE PARSE: {}", card_position);
    let pos: i32 = card_position.parse::<i32>().unwrap();
    println!("POS {}", pos);
    let deleted_card: RedisResult<i32> = redis::cmd("LREM").arg("flashcards").arg(pos).query(&mut connection);

    return deleted_card;
}
