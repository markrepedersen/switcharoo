use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};

#[derive(Serialize, Deserialize, Clone)]
pub struct Permission {
    pub id: i32,
    pub name: String,
}

impl Permission {
    pub async fn get_permissions(pool: &PgPool) -> Result<Vec<Permission>> {
        let permissions = query_as!(Permission, "SELECT * FROM Permissions")
            .fetch_all(pool)
            .await?;

        Ok(permissions)
    }

    pub async fn get_permission(id: i32, pool: &PgPool) -> Result<Permission> {
        let permission = query_as!(Permission, "SELECT * FROM Permissions where id = $1", id)
            .fetch_one(pool)
            .await?;

        Ok(permission)
    }

    pub async fn add_permission(name: String, pool: &PgPool) -> Result<Permission> {
        let permission = query_as!(Permission, "INSERT INTO Permissions(name) VALUES ($1) RETURNING *", name)
            .fetch_one(pool)
            .await?;

        Ok(permission)
    }

    pub async fn update_permission(id: i32, pool: &PgPool) -> Result<Permission> {
        let permission = query_as!(Permission, "UPDATE Permissions SET id = $1 RETURNING *", id)
            .fetch_one(pool)
            .await?;

        Ok(permission)
    }

    pub async fn remove_permission(id: i32, pool: &PgPool) -> Result<Permission> {
        let permission = query_as!(Permission, "DELETE FROM Permissions WHERE id = $1 RETURNING *", id)
            .fetch_one(pool)
            .await?;

        Ok(permission)
    }
}
