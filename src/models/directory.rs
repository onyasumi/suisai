use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Directory {
    pub name: String,
    pub path: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<Thing>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Thing>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryWrapper {

    #[serde(flatten)]
    pub directory: Directory,
    pub id: Thing
    
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Photo {
    #[serde(flatten)]
    pub folder: Directory,
    pub thumbnail: String
}