use std::path::Path;

use sqlx::{Pool, Sqlite, SqlitePool};

pub async fn pre_load_db_into_memory(filename: String) -> Pool<Sqlite> {
	let path = Path::new(&filename);
    if !path.exists() {
        panic!("Database file not found at path: {}. Current directory: {:?}", 
            filename, std::env::current_dir());
    }
    let pool = SqlitePool::connect(":memory:")
    	.await
    	.expect("failed to open in memory db");

    let mut conn = pool.acquire().await.expect("failed to aquire client"); 

    sqlx::query(format!("ATTACH DATABASE 'file:{filename}?mode=ro' AS disk;").as_str())
        .execute(&mut conn)
        .await
        .expect("failed to load db file");

    sqlx::query("CREATE TABLE cards AS SELECT * FROM disk.cards;")
        .execute(&mut conn)
        .await
        .expect("failed to create in memory table");

    sqlx::query("DETACH DATABASE disk;")
        .execute(&mut conn)
        .await
        .expect("failed to unload db file");

    return pool;
}
