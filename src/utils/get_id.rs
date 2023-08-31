use surrealdb::Error;
use surrealdb::Error::Db;
use surrealdb::error::Db::Ignore;
use crate::models::directory::Directory;

pub async fn get_album_id(path: String) -> Result<String, Error> {

    let mut response = crate::DB.query("SELECT id FROM album WHERE path = $path")
        .bind(("path", path)).await?;

    match response.take::<Option<String>>((0, "id"))? {
        Some(s) => Ok(s),
        None => Err(Db(Ignore))
    }

}

pub async fn get_directory_id(dir: Directory) -> Result<String, Error> {

    let album = match dir.album {
        Some(val) => val,
        None => return Err(Db(Ignore))
    };

    let parent = match dir.parent {
        Some(val) => val,
        None => return Err(Db(Ignore))
    };

    let mut response = crate::DB.query("SELECT id FROM folder WHERE name = name AND parent = $parent AND album = $album")
        .bind(("name", dir.name))
        .bind(("parent", parent))
        .bind(("album", album)).await?;

    match response.take::<Option<String>>((0, "id"))? {
        Some(s) => Ok(s),
        None => Err(Db(Ignore))
    }

}