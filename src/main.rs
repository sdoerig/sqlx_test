use sqlx::postgres::PgPoolOptions;
mod db_services;
use db_services::mandants::insert_mandant;

//#[actix_web::main]
#[tokio::main]
// or #[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://doerig:doerig@127.0.2.15/nestbox")
        .await?;
    for _i in 0..5 {
        let _mandant = insert_mandant(&pool).await?;
        println!("{}", _mandant);
    }
    Ok(())
}
