use sqlx::pool::Pool;
use sqlx::postgres::Postgres;
use sqlx::FromRow;
use std::fmt;

const INSERT_MANDANT: &str = "INSERT INTO mandants (association_name, website, email) 
VALUES ($1, $2, $3) returning id::text";

const SELECT_BY_ID_MANDANT: &str =
    "SELECT association_name, website, email from mandants where id = $1::uuid";

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

#[derive(Debug, FromRow)]
struct SelectById {
    pub association_name: String,
    pub website: String,
    pub email: String,
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

    pub async fn select_by_uuid(uuid: String, pool: &Pool<Postgres>) -> Self {
        let select_result = sqlx::query_as::<_, SelectById>(SELECT_BY_ID_MANDANT)
            .bind(&uuid)
            .fetch_one(pool)
            .await;
        match select_result {
            Ok(s) => Mandant::new(uuid, s.association_name, s.website, s.email),
            Err(e) => {
                print!("{}", e);
                Mandant::new(uuid, String::from(""), String::from(""), String::from(""))
            }
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
