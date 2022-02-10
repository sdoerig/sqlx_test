use crate::db_services::db_objects::{DbEntity, PersistenceStatus, PrimaryKey};
use async_trait::async_trait;
use sha3::{Digest, Sha3_256};
use sqlx::pool::Pool;
use sqlx::postgres::Postgres;
use sqlx::FromRow;
use std::default::Default;
use std::fmt;

pub struct User {
    id: String,
    mandants_id: String,
    locked: bool,
    username: String,
    lastname: String,
    email: String,
    password_hash: String,
    salt: String,
}

#[async_trait]
impl DbEntity for User {
    fn primary_key(&self) -> &str {
        todo!()
    }

    fn persistence_status(&self) -> &PersistenceStatus {
        todo!()
    }

    async fn persist(&mut self, pool: &Pool<Postgres>) {
        todo!()
    }

    async fn select(uuid: &str, pool: &Pool<Postgres>) -> Self {
        todo!()
    }
}
