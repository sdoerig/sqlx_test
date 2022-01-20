use sqlx::pool::Pool;
use sqlx::postgres::Postgres;
use sqlx::{Error, FromRow};
use std::fmt;

#[derive(Debug, FromRow)]
pub struct Mandant {
    pub id: String,
    pub association_name: String,
    pub website: String,
    pub email: String,
}

impl fmt::Display for Mandant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "id: {}, association_name: {}, website: {}, email: {}",
            self.id, self.association_name, self.website, self.email
        )
    }
}

const INSERT_MANDANT: &str = "INSERT INTO mandants (association_name, website, email) 
VALUES ($1, $2, $3) returning id::text, association_name, website, email";

pub async fn insert_mandant(db: &Pool<Postgres>) -> Result<Mandant, Error> {
    let insert_query = sqlx::query_as::<_, Mandant>(INSERT_MANDANT)
        .bind("a")
        .bind("b")
        .bind("c")
        .fetch_one(db);
    //sqlx::query_as::<DB, O>(INSERT_MANDANT);
    insert_query.await
}
