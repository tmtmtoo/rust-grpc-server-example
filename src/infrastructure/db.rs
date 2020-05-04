use anyhow::*;
use derive_more::*;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use diesel::{Connection, PgConnection};
use diesel_migrations::RunMigrationsError;
use thiserror::Error;

embed_migrations!();

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

pub fn connection_pool(
    connection_string: impl Into<String>,
    max_size: u32,
) -> Result<ConnectionPool, PoolError> {
    let manager =
        diesel::r2d2::ConnectionManager::<diesel::PgConnection>::new(connection_string.into());

    diesel::r2d2::Pool::builder()
        .max_size(max_size)
        .build(manager)
}

pub fn migration(pool: &ConnectionPool) -> Result<(), RunMigrationsError> {
    let conn = pool.get().map_err(|_| {
        diesel_migrations::RunMigrationsError::MigrationError(
            diesel_migrations::MigrationError::NoMigrationRun,
        )
    })?;

    embedded_migrations::run_with_output(&conn, &mut std::io::stdout())
}

#[derive(AsRef)]
pub struct DbConn<'a>(&'a PgConnection);

#[derive(From, Error, Debug, Display)]
pub enum DbError {
    Handling(diesel::result::Error),
    Connection(r2d2::Error),
}

#[derive(new, Clone)]
pub struct TransactionManager(ConnectionPool);

impl TransactionManager {
    pub fn scope<T, F>(&self, f: F) -> Result<T, DbError>
    where
        F: FnOnce(DbConn) -> Result<T, DbError>,
    {
        let conn = self.0.get()?;
        conn.transaction(|| f(DbConn(&*conn)))
    }
}

#[cfg(test)]
pub fn get_test_transaction_manager() -> TransactionManager {
    use dotenv::dotenv;
    use std::env;
    use std::sync::Once;

    static mut TRANSACTION: Option<TransactionManager> = None;
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            dotenv().ok();
            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let pool = connection_pool(database_url.as_str(), 4).unwrap();
            migration(&pool).unwrap();
            TRANSACTION = Some(TransactionManager::new(pool));
        });

        TRANSACTION.clone().unwrap()
    }
}
