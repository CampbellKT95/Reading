use crate::flashcards;
use crate::interaction;
use crate::connection;
use redis::{RedisResult, RedisError};
extern crate redis;

pub fn add_card(new_card: flashcards::Flashcard ) -> i32 {
    let mut connection = connection::connect();

    // convert newly created card to JSON
    let s_card = serde_json::to_string(&new_card).unwrap();

    let result: RedisResult<i32> = redis::cmd("LPUSH").arg("flashcards").arg(s_card).query(&mut connection);

    // should simply return # of entries (1)
    return result.unwrap();
}

pub fn read_all_cards() -> Vec<String>  {
    let mut connection = connection::connect();

    let all_cards: RedisResult<Vec<String>> = redis::cmd("LRANGE").arg("flashcards").arg(0).arg(-1).query(&mut connection);

    return all_cards.unwrap();
}

// for edit & delete, find the position of the item first with LPOS, then LSET/LREM it
pub fn edit_card(card_position: String) -> RedisResult<()> {
        let mut connection = connection::connect();

        let updated_card: flashcards::Flashcard = interaction::create_card();
        let serialized_card: String = serde_json::to_string(&updated_card).unwrap();

        let _: RedisResult<()> = redis::cmd("LSET").arg("flashcards").arg(card_position).arg(serialized_card).query(&mut connection);

        Ok(())
}

pub fn delete_one_card(card_position: String) -> RedisResult<()> {
    let mut connection = connection::connect();

    let pos: i32 = card_position.parse::<i32>().unwrap();

    // LREM requires the fill list item, not just the index
    // so we need to fetch the item @ given index with LINDEX, then use that as the final arg for LREM
    let el_at_index: Result<String, RedisError> = redis::cmd("LINDEX").arg("flashcards").arg(pos).query(&mut connection);

    let _: RedisResult<i32> = redis::cmd("LREM").arg("flashcards").arg(1).arg(el_at_index.unwrap()).query(&mut connection);

    Ok(())
}
