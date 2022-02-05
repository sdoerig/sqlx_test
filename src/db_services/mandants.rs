use crate::db_services::db_objects::PrimaryKey;

use sha3::{Digest, Sha3_256};
use sqlx::pool::Pool;
use sqlx::postgres::Postgres;
use sqlx::FromRow;
use std::fmt;

const MANDANT_INSERT: &str = "INSERT INTO mandants (association_name, website, email) 
VALUES ($1, $2, $3) returning id::text";

const MANDANT_SELECT_BY_UUID: &str = "SELECT association_name, website, email from mandants 
    where id = $1::uuid";

const MANDANT_UPDATE_BY_UUID: &str =
    "UPDATE mandants set association_name = $1, website = $2, email = $3 
    where id = $4::uuid  returning id::text";

#[derive(Debug, FromRow)]
pub struct Mandant {
    id: String,
    pub association_name: String,
    pub website: String,
    pub email: String,
    hash_value: String
    
}

#[derive(Debug, FromRow)]
struct SelectById {
    pub association_name: String,
    pub website: String,
    pub email: String,
}

fn gen_sha3(tokens: Vec<&str>) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(tokens.join("___"));
    hex::encode(hasher.finalize())
}

impl Mandant {
    pub fn new(association_name: String, website: String, email: String) -> Self {
        let hash_value = gen_sha3(vec![&association_name, &website, &email]);
        Mandant {
            id: String::from(""),
            association_name,
            website,
            email,
            hash_value
        }
    }

    fn map_query_result(
        id: String,
        association_name: String,
        website: String,
        email: String,
    ) -> Self {
        let hash_value = gen_sha3(vec![&association_name, &website, &email]);
        Mandant {
            id,
            association_name,
            website,
            email,
            hash_value
        }
    }

    pub fn primary_key(&self) -> String {
        self.id.clone()
    }

    pub async fn persist(&mut self, pool: &Pool<Postgres>) -> bool {
        if self.id.is_empty() {
            return self.insert(pool).await;
        } else {
            let hash_value = gen_sha3(vec![&self.association_name, &self.website, &self.email]);
            if self.hash_value != hash_value {
                self.hash_value = hash_value;
                return self.update(pool).await;
            }
        }
        false
    }

    async fn insert(&mut self, pool: &Pool<Postgres>) -> bool{
        let insert_result = sqlx::query_as::<_, PrimaryKey>(MANDANT_INSERT)
            .bind(&self.association_name)
            .bind(&self.website)
            .bind(&self.email)
            .fetch_one(pool)
            .await;
        //sqlx::query_as::<DB, O>(INSERT_MANDANT);
        let mut success = true;
        match insert_result {
            Ok(p) => self.id = p.id,
            Err(_e) => success = false,
        }
        success
    }

    pub async fn select(uuid: String, pool: &Pool<Postgres>) -> Self {
        let select_result = sqlx::query_as::<_, SelectById>(MANDANT_SELECT_BY_UUID)
            .bind(&uuid)
            .fetch_one(pool)
            .await;
        match select_result {
            Ok(s) => Mandant::map_query_result(uuid, s.association_name, s.website, s.email),
            Err(_e) => Mandant::map_query_result(
                uuid,
                String::from(""),
                String::from(""),
                String::from(""),
            ),
        }
    }

    async fn update(&mut self, pool: &Pool<Postgres>) -> bool {
        let update_result = sqlx::query_as::<_, PrimaryKey>(MANDANT_UPDATE_BY_UUID)
            .bind(&self.association_name)
            .bind(&self.website)
            .bind(&self.email)
            .bind(&self.id)
            .fetch_one(pool)
            .await;
        match update_result {
            Ok(_s) => true,
            Err(_e) => false,
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

