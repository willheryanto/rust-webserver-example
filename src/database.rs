use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub fn init_pool(database_connection: PgConnectOptions) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(database_connection)
}
