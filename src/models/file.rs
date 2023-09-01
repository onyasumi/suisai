use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::models::directory::Directory;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {

    #[serde(flatten)]
    pub directory: Directory,
    pub thumbnail: String

}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileWrapper {

    #[serde(flatten)]
    pub file: File,
    pub id: Thing

}