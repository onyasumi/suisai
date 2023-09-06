use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::models::file::FileWrapper;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Directory {
    #[serde(default)]
    pub name: String,

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
pub struct DirectoryReturn {

    pub directories: Vec<DirectoryWrapper>,
    pub files: Vec<FileWrapper>

}
