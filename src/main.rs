use sqlx::postgres::PgPoolOptions;
mod db_services;

use crate::db_services::mandants::Mandant;

//#[actix_web::main]
#[tokio::main]
// or #[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://doerig:doerig@127.0.2.15/nestbox")
        .await?;
    for i in 0..5 {
        let mut mandant = Mandant::new(
            String::from(""),
            format!("association {}", i),
            format!("website {}", i),
            format!("email {}", i),
        );
        mandant.insert(&pool).await;
        println!("{}", mandant);
    }
    Ok(())
}
