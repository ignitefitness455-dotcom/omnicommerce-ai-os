use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Initializes a thread-safe Diesel r2d2 PostgreSQL connection pool.
/// Returns `Ok(PgPool)` when `DATABASE_URL` is reachable, or a descriptive error if offline.
pub fn init_postgres_pool(database_url: &str, max_size: u32) -> Result<PgPool, String> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .max_size(max_size)
        .build(manager)
        .map_err(|e| format!("Failed to initialize PostgreSQL connection pool: {}", e))
}
