use db_services::{
    db_objects::{DbEntity, PersistenceStatus},
    user::User,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{fmt::format, time::SystemTime};
mod db_services;

use crate::db_services::mandant::Mandant;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

//#[actix_web::main]
//#[tokio::main]
#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://doerig:doerig@127.0.2.15/nestbox")
        .await?;
    let mut uuids: Vec<Mandant> = Vec::new();
    for i in 0..5 {
        let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(d) => d.as_nanos(),
            Err(_) => 0_u128,
        };

        let mut mandant = Mandant::new(
            format!("association {}", i),
            format!("website {}", i),
            format!("email {} {}", i, now),
        );
        mandant.persist(&pool).await;
        uuids.push(mandant);
        //println!("{}", mandant);
    }

    for (count, mandant) in uuids.into_iter().enumerate() {
        let mut mandant = Mandant::select(mandant.primary_key(), &pool).await;
        println!("{}", &mandant);

        if count % 2 == 0 {
            *mandant.email() = format!("{} {}", String::from("me@me.me"), mandant.email());
            *mandant.association_name() = "gaggger".to_string();
        }

        mandant.persist(&pool).await;

        match mandant.persistence_status() {
            PersistenceStatus::New => {
                println!("Nothing happpend");
            }
            PersistenceStatus::Error(e) => println!("Error {}", e),
            PersistenceStatus::Clean => {
                println!("Content has not changed");
                add_user(&mandant, &pool).await;
            }
        }
    }
    Ok(())
}

async fn add_user(mandant: &Mandant, pool: &Pool<Postgres>) {
    println!("Adding users...");
    for i in 0..5 {
        let mut user = User::new(
            mandant,
            random_str(20),
            format!("firstname {}", i),
            format!("lastname {}", i),
            random_str(32),
            String::from("password"),
        );
        user.persist(pool).await;
        match user.persistence_status() {
            PersistenceStatus::New => println!("User new"),
            PersistenceStatus::Clean => println!("User clean"),
            PersistenceStatus::Error(e) => println!("Error {}", e),
        }
    }
}

fn random_str(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
