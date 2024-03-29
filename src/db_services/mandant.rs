use crate::db_services::db_objects::{gen_sha3, DbEntity, PersistenceStatus, PrimaryKey};
use async_trait::async_trait;
use sqlx::pool::Pool;
use sqlx::postgres::Postgres;
use sqlx::FromRow;
use std::default::Default;
use std::fmt;

/// SQL: Prepared statement used when inserting a record into the table mandants.
const MANDANT_INSERT: &str = "INSERT INTO mandants (association_name, website, email) 
VALUES ($1, $2, $3) returning id::text";

/// SQL: Prepared statement selecting one record from mandants by its UUID which is its primary key.
const MANDANT_SELECT_BY_UUID: &str = "SELECT association_name, website, email from mandants 
    where id = $1::uuid";

/// SQL: Prepared statment updating one records from mandants.
const MANDANT_UPDATE_BY_UUID: &str =
    "UPDATE mandants set association_name = $1, website = $2, email = $3 
    where id = $4::uuid  returning id::text";

#[derive(Debug, FromRow)]
/// Represents one records of the table "mandants".
pub struct Mandant {
    /// Primary key - UUID generated by the database.
    id: String,
    /// Name or the organisation or association.
    association_name: String,
    /// Website of the organisation-
    website: String,
    /// Official email address of the organisation
    email: String,
    /// SHA3 value calculated over the values
    /// - association_name
    /// - website
    /// - email
    ///
    /// This value is primarily used when calling the method
    /// - persist
    ///
    /// The values mentioned are calculted again and if the SHA3 value is different, an update is performed and the new SHA3 value assigned to hash_value.
    hash_value: String,
    persistence_status: PersistenceStatus,
}

#[derive(Debug, FromRow, Default)]
/// Mapping struct used for a select by UUID (id)
struct SelectById {
    pub association_name: String,
    pub website: String,
    pub email: String,
}

impl Mandant {
    /// Constructor mandant
    pub fn new(association_name: String, website: String, email: String) -> Self {
        let hash_value = gen_sha3(vec![&association_name, &website, &email]);
        Mandant {
            id: String::from(""),
            association_name,
            website,
            email,
            hash_value,
            persistence_status: PersistenceStatus::New,
        }
    }

    pub fn association_name(&mut self) -> &mut String {
        &mut self.association_name
    }
    /// Website of the organisation-

    pub fn website(&mut self) -> &mut String {
        &mut self.website
    }
    /// Official email address of the organisation
    pub fn email(&mut self) -> &mut String {
        &mut self.email
    }
    /// Mapping a query result and returing a instance of Mandant.
    fn map_query_result(
        id: String,
        select_res: SelectById,
        persistence_status: PersistenceStatus,
    ) -> Self {
        let hash_value = gen_sha3(vec![
            &select_res.association_name,
            &select_res.website,
            &select_res.email,
        ]);
        Mandant {
            id,
            association_name: select_res.association_name,
            website: select_res.website,
            email: select_res.email,
            hash_value,
            persistence_status,
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

#[async_trait]
impl DbEntity for Mandant {
    fn primary_key(&self) -> &str {
        &self.id
    }

    fn persistence_status(&self) -> &PersistenceStatus {
        &self.persistence_status
    }

    async fn persist(&mut self, pool: &Pool<Postgres>) {
        if self.id.is_empty() {
            self.insert(pool).await;
        } else {
            let hash_value = gen_sha3(vec![&self.association_name, &self.website, &self.email]);
            if self.hash_value != hash_value {
                self.update(pool).await;
                if self.persistence_status() == &PersistenceStatus::Clean {
                    self.hash_value = hash_value;
                }
            } else {
                self.persistence_status = PersistenceStatus::Clean;
            }
        }
    }

    async fn select(uuid: &str, pool: &Pool<Postgres>) -> Self {
        let select_result = sqlx::query_as::<_, SelectById>(MANDANT_SELECT_BY_UUID)
            .bind(&uuid)
            .fetch_one(pool)
            .await;
        match select_result {
            Ok(s) => Mandant::map_query_result(uuid.to_string(), s, PersistenceStatus::Clean),
            Err(e) => Mandant::map_query_result(
                uuid.to_string(),
                SelectById::default(),
                PersistenceStatus::Error(format!("{}", e)),
            ),
        }
    }

    async fn insert(&mut self, pool: &Pool<Postgres>) {
        let insert_result = sqlx::query_as::<_, PrimaryKey>(MANDANT_INSERT)
            .bind(&self.association_name)
            .bind(&self.website)
            .bind(&self.email)
            .fetch_one(pool)
            .await;
        //sqlx::query_as::<DB, O>(INSERT_MANDANT);
        match insert_result {
            Ok(p) => {
                self.id = p.id;
                self.persistence_status = PersistenceStatus::Clean;
            }
            Err(e) => self.persistence_status = PersistenceStatus::Error(format!("{}", e)),
        }
    }

    async fn update(&mut self, pool: &Pool<Postgres>) {
        let update_result = sqlx::query_as::<_, PrimaryKey>(MANDANT_UPDATE_BY_UUID)
            .bind(&self.association_name)
            .bind(&self.website)
            .bind(&self.email)
            .bind(&self.id)
            .fetch_one(pool)
            .await;
        match update_result {
            Ok(_s) => self.persistence_status = PersistenceStatus::Clean,
            Err(e) => self.persistence_status = PersistenceStatus::Error(format!("{}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;

    const DB_URL: &str = "postgres://doerig:doerig@127.0.2.15/nestbox";
    const MANDANT_VALID_UUID: &str = "e451186c-0dbe-4496-93d6-aa14cac9be31";
    const MANDANT_INVALID_UUID: &str = "00000000-0000-0000-0000-000000000000";
    const ASSOCIATION_NAME_1: &str = "association_name1";
    const WEBSITE_NAME_1: &str = "website1";
    const EMAIL_NAME_1: &str = "email1";
    const ASSOCIATION_NAME_2: &str = "association_name2";
    const WEBSITE_NAME_2: &str = "website2";
    const EMAIL_NAME_2: &str = "email2";

    #[actix_rt::test]
    async fn create_mandant_check_field_modifications() {
        let mut mandant = Mandant::new(
            ASSOCIATION_NAME_1.to_string(),
            WEBSITE_NAME_1.to_string(),
            EMAIL_NAME_1.to_string(),
        );

        // test initial assigment
        assert!(mandant.persistence_status() == &PersistenceStatus::New);
        assert!(
            mandant.association_name() == ASSOCIATION_NAME_1,
            "association_name is {} expected {}",
            mandant.association_name(),
            ASSOCIATION_NAME_1
        );
        assert!(
            mandant.website() == WEBSITE_NAME_1,
            "website is {} expected {}",
            mandant.website(),
            WEBSITE_NAME_1
        );
        assert!(
            mandant.email() == EMAIL_NAME_1,
            "email is {} expected {}",
            mandant.email(),
            EMAIL_NAME_1
        );
        // modificate
        for [association, website, email] in [
            [ASSOCIATION_NAME_2, WEBSITE_NAME_2, EMAIL_NAME_2],
            [ASSOCIATION_NAME_1, WEBSITE_NAME_1, EMAIL_NAME_1],
        ] {
            *mandant.association_name() = association.to_string();
            *mandant.website() = website.to_string();
            *mandant.email() = email.to_string();
            assert!(mandant.persistence_status() == &PersistenceStatus::New);
            assert!(
                mandant.association_name() == association,
                "association_name is {} expected {}",
                mandant.association_name(),
                association
            );
            assert!(
                mandant.website() == website,
                "website is {} expected {}",
                mandant.website(),
                website
            );
            assert!(
                mandant.email() == email,
                "email is {} expected {}",
                mandant.email(),
                email
            );
        }
    }

    #[actix_rt::test]
    async fn select_valid_mandant() {
        if let Ok(pool) = &get_pool().await {
            let mandant = Mandant::select(MANDANT_VALID_UUID, pool).await;
            assert!(mandant.persistence_status() == &PersistenceStatus::Clean)
        } else {
            panic!("Could not get PostgreSQL pool...")
        }
    }

    #[actix_rt::test]
    async fn select_invalid_mandant() {
        if let Ok(pool) = &get_pool().await {
            let mandant = Mandant::select(MANDANT_INVALID_UUID, pool).await;
            match mandant.persistence_status() {
                PersistenceStatus::New => panic!("Record can not be New"),
                PersistenceStatus::Clean => panic!("Record can not be Clean"),
                PersistenceStatus::Error(_) => assert!(true),
            };

            //assert!(mandant.persistence_status() == &PersistenceStatus::Error("".to_string()))
        } else {
            panic!("Could not get PostgreSQL pool...")
        }
    }

    async fn get_pool() -> Result<Pool<Postgres>, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(DB_URL)
            .await;
        pool
    }
}
