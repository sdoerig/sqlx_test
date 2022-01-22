use sqlx::pool::Pool;
use sqlx::postgres::Postgres;
use sqlx::{Error, FromRow};
use std::fmt;

const INSERT_MANDANT: &str = "INSERT INTO mandants (association_name, website, email) 
VALUES ($1, $2, $3) returning id::text";

#[derive(Debug, FromRow)]
pub struct Mandant {
    pub id: String,
    pub association_name: String,
    pub website: String,
    pub email: String,
}

#[derive(Debug, FromRow)]
struct PrimaryKey {
    pub id: String,
}

impl Mandant {
    pub fn new(id: String, association_name: String, website: String, email: String) -> Self {
        Mandant {
            id,
            association_name,
            website,
            email,
        }
    }

    pub async fn insert(&mut self, pool: &Pool<Postgres>) {
        let insert_result = sqlx::query_as::<_, PrimaryKey>(INSERT_MANDANT)
            .bind(&self.association_name)
            .bind(&self.website)
            .bind(&self.email)
            .fetch_one(pool)
            .await;
        //sqlx::query_as::<DB, O>(INSERT_MANDANT);
        self.id = match insert_result {
            Ok(p) => p.id,
            Err(_) => String::from(""),
        }
    }
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
