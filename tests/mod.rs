extern crate jservice;

#[test]
fn deserialize_clue() {
    extern crate serde_json;

    let bytes = r#"{"id":1,"answer":"Election Day","question":"1st Tuesday after the 1st Monday in November","value":100,"airdate":"1985-02-08T12:00:00.000Z","created_at":"2014-02-11T22:47:18.786Z","updated_at":"2014-02-11T22:47:18.786Z","category_id":1,"game_id":null,"invalid_count":null,"category":{"id":1,"title":"politics","created_at":"2014-02-11T22:47:18.687Z","updated_at":"2014-02-11T22:47:18.687Z","clues_count":30}}"#;
    let clue: Clue = serde_json::from_str(bytes).unwrap();
    assert_eq!(clue.question, "1st Tuesday after the 1st Monday in November");
}

#[test]
fn get_clues() {
    jservice::get_clues(None, None, None, None, None).unwrap();
}

#[test]
fn get_random() {
    jservice::get_random(None).unwrap();
}

#[test]
fn get_categories() {
    jservice::get_categories(None, None).unwrap();
}

#[test]
fn get_category() {
    jservice::get_category(1).unwrap();
}
