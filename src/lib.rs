//! # jservice-rs
//!
//! jservice-rs is an API wrapper for [jService](http://jservice.io/).
#![doc(html_root_url = "https://vectorious.github.io/jservice-rs/")]

extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate chrono;

include!(concat!(env!("OUT_DIR"), "/lib.rs"));
