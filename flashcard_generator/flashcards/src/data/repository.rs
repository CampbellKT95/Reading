use crate::flashcards;
use redis::{RedisResult, Iter};
extern crate redis;
// use serde::{Serialize, Serializer};

pub fn add_card(connection: &mut redis::Connection, new_card: flashcards::Flashcard ) -> i32 {
    // convert newly created card to JSON
    let s_card = serde_json::to_string(&new_card).unwrap();

    let result: RedisResult<i32> = redis::cmd("LPUSH").arg("flashcards").arg(s_card).query(connection);

    // should simply return # of entries (1)
    return result.unwrap();
}

pub fn read_all_cards(connection: &mut redis::Connection) -> Iter<String> {
    let all_cards: redis::Iter<String> = redis::cmd("LRANGE").arg("flashcards").arg(0).arg(-1).clone().iter(connection).unwrap();

    return all_cards;
}

// for edit & delete, find the position of the item first with LPOS, then LSET/LREM it
pub fn edit_card(connection: &mut redis::Connection) {}

pub fn delete_one_card(connection: &mut redis::Connection) {}