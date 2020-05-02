use anyhow::*;
use derive_more::*;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use diesel::{Connection, PgConnection};
use diesel_migrations::RunMigrationsError;
use thiserror::Error;

embed_migrations!();

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

pub fn connection_pool(
    connection_string: &str,
    max_size: u32,
) -> Result<ConnectionPool, PoolError> {
    let manager = diesel::r2d2::ConnectionManager::<diesel::PgConnection>::new(connection_string);

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
