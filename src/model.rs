use chrono::*;

#[derive(Deserialize)]
pub struct Clue {
    pub id: u64,
    pub answer: String,
    pub question: String,
    pub value: Option<i32>,
    pub airdate: DateTime<UTC>,
    pub created_at: Option<DateTime<UTC>>,
    pub updated_at: Option<DateTime<UTC>>,
    pub category_id: u64,
    pub game_id: Option<u64>,
    pub invalid_count: Option<u32>,
    pub category: Option<Category>,
}

#[derive(Deserialize)]
pub struct Category {
    pub id: u64,
    pub title: String,
    pub created_at: Option<DateTime<UTC>>,
    pub updated_at: Option<DateTime<UTC>>,
    pub clues_count: u32,
    pub clues: Option<Vec<Clue>>,
}
