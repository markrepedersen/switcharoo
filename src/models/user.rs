use anyhow::Result;
use argon2::ThreadMode;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

use crate::routes::users::UserRequest;

use super::permission::Permission;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}

impl From<UserRequest> for User {
    fn from(user: UserRequest) -> Self {
        User {
            id: Uuid::new_v4(),
            email: user.email,
            password: user.password,
        }
    }
}

#[allow(dead_code)]
impl User {
    pub async fn create(user: UserRequest, pool: &PgPool) -> Result<()> {
        let user = Self::from(user);

        query!(
            "INSERT INTO Users (id, email, password) VALUES ($1, $2, $3)",
            user.id,
            user.email,
            Self::hash_password(&user.password)?,
        )
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn is_signed_in(user: &UserRequest, pool: &PgPool) -> Result<User> {
        let db_user = User::find_by_email(&user.email, pool).await?;

        db_user.verify_password(&user.password)?;

        Ok(db_user)
    }

    pub fn hash_password(password: &String) -> Result<String> {
        let salt: [u8; 32] = thread_rng().gen();
        let mut config = argon2::Config::default();

        config.lanes = 4;
        config.thread_mode = ThreadMode::Parallel;
        config.hash_length = 32;

        let hashpass = argon2::hash_encoded(password.as_bytes(), &salt, &config)?;

        Ok(hashpass)
    }

    pub fn verify_password(&self, password: &String) -> Result<bool> {
        let is_valid = argon2::verify_encoded(&self.password, password.as_bytes())?;

        Ok(is_valid)
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<User>> {
        let users: Vec<User> = query_as!(User, "SELECT * FROM Users ORDER BY id")
            .fetch_all(pool)
            .await?;

        Ok(users)
    }

    pub async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<User> {
        let user = query_as!(User, "SELECT * FROM Users WHERE id = $1", id)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    pub async fn find_by_email(email: &String, pool: &PgPool) -> Result<User> {
        let user = query_as!(User, "SELECT * FROM Users WHERE email = $1", email)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    pub async fn update(id: Uuid, user: UserRequest, pool: &PgPool) -> Result<()> {
        query!(
            "UPDATE Users SET email = $1, password = $2 WHERE id = $3",
            user.email,
            Self::hash_password(&user.password)?,
            id,
        )
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn delete(id: Uuid, pool: &PgPool) -> Result<()> {
        query!("DELETE FROM Users WHERE id = $1", id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn get_user_permissions(id: Uuid, pool: &PgPool) -> Result<Vec<Permission>> {
        let permissions = query_as!(
            Permission,
            "
SELECT Permissions.id, Permissions.name 
FROM UserPermissions INNER JOIN Permissions
ON UserPermissions.permission_id = Permissions.id
WHERE UserPermissions.user_id = $1",
            id
        )
            .fetch_all(pool)
            .await?;

        Ok(permissions)
    }
}
