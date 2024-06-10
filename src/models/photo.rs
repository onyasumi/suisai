use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;


// An album is a pointer to a directory
#[derive(Debug, Serialize, Deserialize)]
pub struct Photo {
    pub label: String,
    pub path: String,
    pub date: String,
    pub tags: Vec<String>,
    pub id: Thing
}
