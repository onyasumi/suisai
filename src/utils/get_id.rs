use surrealdb::Error;
use surrealdb::Error::Db;
use surrealdb::error::Db::Ignore;
use surrealdb::sql::Thing;

pub async fn get_album_id(path: String) -> Result<String, Error> {

    let mut response = crate::DB.query("SELECT id FROM album WHERE path = $path")
        .bind(("path", path)).await?;

    match response.take::<Option<String>>((0, "id"))? {
        Some(s) => Ok(s),
        None => Err(Db(Ignore))
    }

}

pub async fn get_directory_id(path: String, album: Thing) -> Result<String, Error> {

    let mut response = crate::DB.query("SELECT id FROM folder WHERE path = $path AND album = $album")
        .bind(("path", path))
        .bind(("album", album)).await?;

    match response.take::<Option<String>>((0, "id"))? {
        Some(s) => Ok(s),
        None => Err(Db(Ignore))
    }

}