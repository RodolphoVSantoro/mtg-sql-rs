use std::path::Path;

use sqlx::{Pool, Sqlite, SqlitePool, sqlite::SqliteConnectOptions};

fn check_file(file_name: &str) {
    let path = Path::new(&file_name);
    assert!(path.exists(),
         "Database file not found at path: {}. Current directory: {:?}",
         file_name,
         std::env::current_dir()
    );
}

pub async fn pre_load_db_into_memory(file_name: String) -> Pool<Sqlite> {
    check_file(&file_name);
    let pool = SqlitePool::connect(":memory:")
        .await
        .expect("failed to open in memory db");

    let mut conn = pool.acquire().await.expect("failed to aquire client");

    sqlx::query(format!("ATTACH DATABASE 'file:{file_name}?mode=ro' AS disk;").as_str())
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

pub async fn create_pool_to_disk_file(file_name: String) -> Pool<Sqlite> {
    check_file(&file_name);
    let options = SqliteConnectOptions::new()
        .filename(file_name)
        .read_only(true);

    return SqlitePool::connect_with(options)
        .await
        .expect("failed to connect to sqlite db file");
}
