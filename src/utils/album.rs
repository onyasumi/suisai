use surrealdb::Error;
use surrealdb::Error::Db;
use surrealdb::error::Db::Ignore;

pub async fn album_id_from_path(path: String) -> Result<String, Error> {

    let mut response = crate::DB.query("SELECT id FROM album WHERE id = $path")
        .bind(("path", path)).await?;

    match response.take::<Option<String>>((0, "id"))? {
        Some(s) => Ok(s),
        None => Err(Db(Ignore))
    }

}