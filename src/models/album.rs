use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    pub label: String,
    pub path: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Thing>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumWrapper {
    #[serde(flatten)]
    pub album: Album,
    pub id: String
}

// TODO: Replace wrappers with skip_deserializing macro