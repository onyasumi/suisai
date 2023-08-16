use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    pub label: String,
    pub path: String,

    #[serde(default)]
    pub children: Vec<Thing>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Thing>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumWrapper {
    pub album: Album,
    pub id: String
}