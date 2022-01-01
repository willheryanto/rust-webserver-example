use crate::modules::user::database::port::UserRepositoryPort;
use crate::modules::user::domain::entities::user_entity::UserEntity;
use crate::modules::user::domain::value_objects::password_hash::PasswordHash;
use crate::modules::user::domain::value_objects::id::Id;
use crate::settings::get_settings;
use crate::database::init_pool;

use async_trait::async_trait;
use sqlx::PgPool;
use secrecy::Secret;
use uuid::Uuid;
use std::io::{Error, ErrorKind};

pub struct PostgresAdapter {
    pub pool: PgPool
}

impl PostgresAdapter {
    pub fn new() -> Self {
        let settings = get_settings().expect("Failed to get settings");
        let pool = init_pool(settings.database.with_db());

        Self {
            pool
        }
    }
}

#[async_trait(?Send)]
impl UserRepositoryPort for PostgresAdapter {
    //async fn save(&self, entity: UserEntity) -> Result<Id, std::io::Error> {
        //let id = sqlx::query!(
            //r#"
            //INSERT INTO users (id, username, password_hash)
            //VALUES ($1, $2, $3)
            //RETURNING id
            //"#,
            //entity.id.as_ref(),
            //entity.username,
            //entity.password_hash.as_ref()
        //)
        //.fetch_one(&self.pool)
        //.await
        //.expect("Failed to save user")
        //.id;

        //Ok(Id(id))
    //}

    async fn save(&self, entity: UserEntity) -> Result<Id, std::io::Error> {
        match sqlx::query!(
            r#"
            INSERT INTO users (id, username, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
            entity.id.as_ref(),
            entity.username,
            entity.password_hash.as_ref()
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(row) => return Ok(Id(row.id)),
            Err(e) => {
                println!("{}", e);
                return Err(Error::new(ErrorKind::Other, "Failed to save user"));
            }
        };
    }

    async fn find_one_by_username(&self, username: String) -> Result<UserEntity, std::io::Error> {
        let row = sqlx::query!(
            r#"
            SELECT
                id,
                username,
                password_hash
            FROM users
            WHERE username = $1
            "#, 
            username
        )
        .fetch_one(&self.pool)
        .await
        .expect("Failed to find query");

        Ok(UserEntity {
            id: Id(row.id),
            username: row.username,
            password_hash: PasswordHash(Secret::new(row.password_hash))
        })
    }

    async fn find_one_by_id(&self, id: Uuid) -> Result<UserEntity, std::io::Error> {
        let row = sqlx::query!(
            r#"
            SELECT
                id,
                username,
                password_hash
            FROM users
            WHERE id = $1
            "#, 
            id
        )
        .fetch_one(&self.pool)
        .await
        .expect("Failed to find query");

        Ok(UserEntity {
            id: Id(row.id),
            username: row.username,
            password_hash: PasswordHash(Secret::new(row.password_hash))
        })
    }
}
