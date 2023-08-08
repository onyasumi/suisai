use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    pub label: String,
    pub path: String,
    pub children: Vec<Thing>,
    pub owner: Thing,
    pub id: Thing
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumHTTP {
    pub label: String,
    pub path: String,
    pub id: String
}