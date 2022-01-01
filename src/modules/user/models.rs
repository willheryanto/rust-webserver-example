use sqlx::PgPool;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
}

impl User {
    pub async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<Option<User>, sqlx::Error> {
        let res = sqlx::query_as!(
            User,
            r#"
            SELECT
                id
            FROM users
            WHERE id = $1
            "#, 
            id
        )
        .fetch_optional(pool)
        .await?;

        match res {
            Some(user) => Ok(Some(user)),
            None => Ok(None),
        }
    }
}

