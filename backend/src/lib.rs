#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery
)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::indexing_slicing,
    clippy::as_conversions,
    clippy::cast_lossless
)]

use serde::{Serialize, Deserialize};

pub mod db;

#[derive(Serialize, Deserialize)]
pub struct Prompt {
    pub id: Option<u64>,
    pub title: String,
    pub segments: Box<[(String, String)]>,
}
