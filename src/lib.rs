//! # jservice-rs
//!
//! jservice-rs is an API wrapper for [jService](http://jservice.io/).
#![doc(html_root_url = "https://vectorious.github.io/jservice-rs/")]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate chrono;

use chrono::*;
use hyper::{Client, Url};

const API_URI: &'static str = "http://jservice.io/api/";

/// Get clues.
///
/// Returns clues with `category` field.
///
/// * `value` - the value of the clue in dollars
/// * `category` - the id of the category you want to return
/// * `min_date` - earliest date to show, based on original air date
/// * `max_date` - latest date to show, based on original air date
/// * `offset` - offsets the returned clues. Useful in pagination
pub fn get_clues(value: Option<i32>,
                 category: Option<u32>,
                 min_date: Option<DateTime<UTC>>,
                 max_date: Option<DateTime<UTC>>,
                 offset: Option<u32>)
                 -> Result<Vec<Clue>, String>
{
    let mut params: Vec<(&'static str, String)> = Vec::new();
    if value.is_some() {
        params.push(("value", value.unwrap().to_string()));
    }
    if category.is_some() {
        params.push(("category", category.unwrap().to_string()));
    }
    if min_date.is_some() {
        params.push(("min_date", min_date.unwrap().to_string()));
    }
    if max_date.is_some() {
        params.push(("max_date", max_date.unwrap().to_string()));
    }
    if offset.is_some() {
        params.push(("offset", offset.unwrap().to_string()));
    }

    match get_api("clues", &params) {
        Ok(resp) => {
            parse_response(&resp)
        }
        Err(e) => { Err(e) }
    }
}

/// Get random clues.
///
/// Returns clues with `category` field.
///
/// * `count` - amount of clues to return, limited to 100 at a time. 1 by default.
pub fn get_random(count: Option<u32>) -> Result<Vec<Clue>, String> {
    let mut params: Vec<(&'static str, String)> = Vec::new();
    if count.is_some() {
        params.push(("count", count.unwrap().to_string()));
    }

    match get_api("random", &params) {
        Ok(resp) => {
            parse_response(&resp)
        }
        Err(e) => { Err(e) }
    }
}

/// Get categories.
///
/// Does not return categories with `clues` field.
///
/// * `count` - amount of categories to return, limited to 100 at a time. 1 by default.
/// * `offset` - offsets the starting id of categories returned. Useful in pagination.
pub fn get_categories(count: Option<u32>, offset: Option<u32>) -> Result<Vec<Category>, String> {
    let mut params: Vec<(&'static str, String)> = Vec::new();
    if count.is_some() {
        params.push(("count", count.unwrap().to_string()));
    }
    if offset.is_some() {
        params.push(("offset", offset.unwrap().to_string()));
    }

    match get_api("categories", &params) {
        Ok(resp) => {
            parse_response(&resp)
        }
        Err(e) => { Err(e) }
    }
}

/// Get category.
///
/// Returns category with `clues` field.
///
/// * `id` - *Required* the ID of the category to return.
pub fn get_category(id: u64) -> Result<Category, String> {
    match get_api("category", &vec![("id", id.to_string())]) {
        Ok(resp) => {
            parse_response(&resp)
        }
        Err(e) => { Err(e) }
    }
}

fn parse_response<T>(resp: &str) -> Result<T, String> where T: serde::de::DeserializeOwned {
    match serde_json::from_str(&resp) {
        Ok(t) => { Ok(t) }
        Err(e) => { Err(format!("{:?}", e)) }
    }
}

fn get_api(url_extension: &str, params: &[(&'static str, String)]) -> Result<String, String> {
    let client = Client::new();
    let mut url = Url::parse(API_URI).unwrap().join(url_extension).unwrap();
    url.query_pairs_mut().extend_pairs(params);
    let req = client.get(url);

    match req.send() {
        Ok(mut res) => {
            use std::io::Read;

            let mut body: Vec<u8> = Vec::new();
            match res.read_to_end(&mut body) {
                Ok(_) => { Ok(String::from_utf8(body).unwrap()) }
                Err(e) => { Err(format!("{}", e)) }
            }
        }
        Err(e) => { Err(format!("{:?}", e)) }
    }
}

/// A jService clue.
#[derive(Deserialize, Clone)]
pub struct Clue {
    /// The ID of the clue.
    pub id: u64,
    /// The answer of the clue.
    pub answer: String,
    /// The question of the clue.
    pub question: String,
    /// The dollar value of the clue. If None, clue is a Daily Double.
    pub value: Option<i32>,
    /// The original air date of the clue.
    pub airdate: DateTime<UTC>,
    /// When the clue was added to the database.
    pub created_at: Option<DateTime<UTC>>,
    /// When the clue was last updated.
    pub updated_at: Option<DateTime<UTC>>,
    /// The ID of the category of the clue.
    pub category_id: u64,
    pub game_id: Option<u64>,
    /// How many times the clue has been marked invalid by users.
    pub invalid_count: Option<u32>,
    /// The category of the clue.
    pub category: Option<Category>,
}

/// A jService category.
#[derive(Deserialize, Clone)]
pub struct Category {
    /// The ID of the category.
    pub id: u64,
    /// The title of the category.
    pub title: String,
    /// When the category was added to the database.
    pub created_at: Option<DateTime<UTC>>,
    /// When the category was last updated.
    pub updated_at: Option<DateTime<UTC>>,
    /// How many clues belong to the category.
    pub clues_count: u32,
    /// All of the clues that belong to this category.
    pub clues: Option<Vec<Clue>>,
}
