use {
    log::info,
    sea_orm::{ConnectOptions, DatabaseConnection, DbErr},
};

pub mod user;

pub async fn get_db(url: &str, sqlx_logs: bool) -> Result<DatabaseConnection, DbErr> {
    info!("Connecting to database at {}", url);

    let mut opts = ConnectOptions::new(url);
    opts.sqlx_logging(sqlx_logs);

    // connection pool settings
    opts.max_connections(10)
        .min_connections(1)
        .connect_timeout(std::time::Duration::from_secs(10))
        .idle_timeout(std::time::Duration::from_secs(600));

    let db = sea_orm::Database::connect(opts).await?;

    // Execute SQLite-specific performance optimizations
    use sea_orm::ConnectionTrait;
    db.execute_unprepared("PRAGMA journal_mode = WAL;").await?;
    db.execute_unprepared("PRAGMA synchronous = NORMAL;").await?;
    db.execute_unprepared("PRAGMA cache_size = -64000;").await?; // 64MB cache
    db.execute_unprepared("PRAGMA temp_store = MEMORY;").await?;
    db.execute_unprepared("PRAGMA mmap_size = 268435456;").await?; // 256MB mmap

    Ok(db)
}
