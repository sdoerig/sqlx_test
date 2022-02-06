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
