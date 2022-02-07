use db_services::db_objects::{DbEntity, PersistenceStatus};
use sqlx::postgres::PgPoolOptions;
use std::time::SystemTime;
mod db_services;

use crate::db_services::db_model::Mandant;

//#[actix_web::main]
//#[tokio::main]
#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://doerig:doerig@127.0.2.15/nestbox")
        .await?;
    let mut uuids: Vec<String> = Vec::new();
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
        uuids.push(mandant.primary_key().to_string());
        println!("{}", mandant);
    }

    for (count, uuid) in uuids.into_iter().enumerate() {
        let mut mandant = Mandant::select(uuid, &pool).await;
        println!("{}", &mandant);

        if count % 2 == 0 {
            mandant.email = format!("{} {}", String::from("me@me.me"), mandant.email);
        }

        mandant.persist(&pool).await;

        match mandant.persistence_status() {
            PersistenceStatus::New => println!("Nothing happpend"),
            PersistenceStatus::Success => println!("Success"),
            PersistenceStatus::Error(e) => println!("Error {}", e),
            PersistenceStatus::Clean => println!("Content has not changed"),
        }
    }
    Ok(())
}
