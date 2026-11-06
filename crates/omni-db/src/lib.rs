//! `omni-db`: Diesel ORM schema and repository models for PostgreSQL.

pub mod models;
pub mod pool;
pub mod schema;

pub use models::*;
pub use pool::{init_postgres_pool, PgPool};
