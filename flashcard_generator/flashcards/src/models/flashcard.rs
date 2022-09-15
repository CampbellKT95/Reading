use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Flashcard {
    pub word: String,
    pub furigana: String,
    pub definition: String,
    pub example: String,
    pub difficulty: i32,
    pub last_reviewed: String
}

#[derive(Debug)]
pub struct CardPosition {
    pub position: i32,
    pub word: String
}