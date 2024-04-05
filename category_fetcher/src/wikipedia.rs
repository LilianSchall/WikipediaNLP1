use serde;

use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
pub struct Article {
    #[serde(rename = "")]
    pub line_number: i32,
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Category {
    pub ns: i32,
    pub title: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Page {
    pub pageid: i32,
    pub ns: i32,
    pub title: String,
    pub categories: Vec<Category>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Query {
    pub pages: HashMap<String, Page>,
}

#[derive(Debug, serde::Deserialize)]
pub struct WikipediaResponse {
    pub batchcomplete: String,
    pub query: Query,
    pub limits: HashMap<String, i32>,
}
