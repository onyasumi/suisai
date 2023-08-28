use surrealdb::Error;
use surrealdb::sql::Thing;
use async_recursion::async_recursion;

#[async_recursion]
pub async fn delete_recursive(id: Thing) -> Result<i8, Error> {

    // TODO: Delete all files here

    // Get all subdirectories
    let subdirs: Vec<Thing> = crate::DB.query("SELECT id FROM directory WHERE parent = $parent")
        .bind(("parent", id.clone())).await?.take(0)?;

    // Recurse for all subdirectories
    for dir in subdirs {
        delete_recursive(dir).await?;
    }

    // Delete current directory
    crate::DB.delete(("directory", id)).await?;
    
    Ok(69)

}