use sqlx::postgres::PgPoolOptions;
mod db_services;

use crate::db_services::mandants::Mandant;

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
        let mut mandant = Mandant::new(
            format!("association {}", i),
            format!("website {}", i),
            format!("email {}", i),
        );
        mandant.persist(&pool).await;
        uuids.push(mandant.primary_key());
        println!("{}", mandant);
    }

    for uuid in uuids {
        let mut mandant = Mandant::select(uuid, &pool).await;
        println!("{}", &mandant);
        mandant.email = String::from("me@me.me");
        if mandant.persist(&pool).await {
            println! {"Successfully updated: mandant {}", mandant};
        }
    }
    Ok(())
}
