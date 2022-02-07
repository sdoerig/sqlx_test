use async_trait::async_trait;
use sqlx::pool::Pool;
use sqlx::postgres::Postgres;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct PrimaryKey {
    pub id: String,
}

#[derive(Debug, PartialEq)]
/// Holds the information about any database interaction.
pub enum PersistenceStatus {
    /// New means the for a just created not yet persited record.
    New,
    /// Clean means the record is persisted but its content has not
    /// been modified by the application.
    Clean,
    /// Success means teh record has successfully been persisted
    /// in the database.
    Success,
    /// Error, something when wrong - the string contains a user
    /// readable description of the error.
    Error(String),
}

#[async_trait]

pub trait DbEntity {
    /// Returing a reference of the primary key.
    fn primary_key(&self) -> &str;

    /// Returning a reference of the persistence status.
    fn persistence_status(&self) -> &PersistenceStatus;

    /// Persisting the tupel to the database. It will either perform
    /// an insert or an update.
    /// An insert is performed if the id (primary key) is empty and an update is performed
    /// if the SHA3 value newly calculated is different.
    async fn persist(&mut self, pool: &Pool<Postgres>);
    async fn select(uuid: String, pool: &Pool<Postgres>) -> Self;
}
