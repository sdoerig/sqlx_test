use crate::db_services::db_objects::{DbEntity, PersistenceStatus, PrimaryKey};
use async_trait::async_trait;
use sha3::{Digest, Sha3_256};
use sqlx::pool::Pool;
use sqlx::postgres::Postgres;
use sqlx::FromRow;
use std::default::Default;
use std::fmt;

use super::db_objects::gen_sha3;
use super::mandant::Mandant;

const USER_INSERT: &str = "insert into users (mandants_id, locked, username,
    firstname, lastname, email, password_hash ) 
    values ($1, $2, $3, $4, $5, $6, crypt($7, gen_salt('md5'))) returning id::text;";

const USER_IS_LOGGED_IN: &str = "select 
* 
from 
users 
where 
(password_hash = crypt('password', password_hash)) = true and 
username = 'tracy';";

pub struct User {
    id: String,
    mandant_id: String,
    locked: bool,
    username: String,
    firstname: String,
    lastname: String,
    email: String,
    password_hash: String,
    hash_value: String,
}

#[async_trait]
impl DbEntity for User {
    fn primary_key(&self) -> &str {
        &self.id
    }

    fn persistence_status(&self) -> &PersistenceStatus {
        todo!()
    }

    async fn persist(&mut self, pool: &Pool<Postgres>) {
        if self.id.is_empty() {}
    }

    async fn select(uuid: &str, pool: &Pool<Postgres>) -> Self {
        todo!()
    }

    async fn insert(&mut self, pool: &Pool<Postgres>) {
        todo!()
    }

    async fn update(&mut self, pool: &Pool<Postgres>) {
        todo!()
    }
}

impl User {
    pub fn new(
        mandant: &Mandant,
        username: String,
        firstname: String,
        lastname: String,
        email: String,
        password_hash: String,
    ) -> Self {
        let mandant_id = mandant.primary_key().to_string();
        let hash_value = gen_sha3(vec![
            &mandant_id,
            &username,
            &firstname,
            &lastname,
            &email,
            &password_hash,
        ]);
        User {
            id: String::from(""),
            mandant_id,
            username,
            locked: false,
            firstname,
            lastname,
            email,
            password_hash,
            hash_value,
        }
    }
}
