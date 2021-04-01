use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, PgPool};

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub hashpass: String,
}

#[allow(dead_code)]
impl User {
    pub async fn has_password(user: UserRequest, pool: &PgPool) -> Result<bool> {
        let db_user = Self::find_by_email(user.email, pool).await?;

        Ok(match verify(&db_user.hashpass, &user.password) {
            Ok(true) => true,
            _ => false,
        })
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<User>> {
        let users: Vec<User> = query_as!(User, "SELECT * FROM Users ORDER BY id")
            .fetch_all(pool)
            .await?;

        Ok(users)
    }

    pub async fn find_by_id(id: i32, pool: &PgPool) -> Result<User> {
        let user = query_as!(User, "SELECT * FROM Users WHERE id = $1", id)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    pub async fn find_by_email(email: String, pool: &PgPool) -> Result<User> {
        let user = query_as!(User, "SELECT * FROM Users WHERE email = $1", email)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    pub async fn create(user: UserRequest, pool: &PgPool) -> Result<()> {
        let hashpass = hash(&user.password, DEFAULT_COST)?;

        query!(
            "INSERT INTO Users (email, hashpass) VALUES ($1, $2)",
            user.email,
            hashpass,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update(id: i32, user: UserRequest, pool: &PgPool) -> Result<()> {
        let hashpass = hash(&user.password, DEFAULT_COST)?;

        query!(
            "UPDATE Users SET email = $1, hashpass = $2 WHERE id = $3",
            user.email,
            hashpass,
            id,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<()> {
        query!("DELETE FROM Users WHERE id = $1", id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
