use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Permission {
    pub id: i32,
    pub name: String,
}

impl Permission {
    pub async fn get_permissions(pool: &PgPool) -> Result<Vec<Permission>> {
        let permissions: Vec<Permission> = query_as!(Permission, "SELECT * FROM Permissions")
            .fetch_all(pool)
            .await?;

        Ok(permissions)
    }

    pub async fn add_permission(name: String, pool: &PgPool) -> Result<()> {
        query!("INSERT INTO Permissions(name) VALUES ($1)", name)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn remove_permission(name: String, pool: &PgPool) -> Result<()> {
        query!("DELETE FROM Permissions WHERE name = $1", name)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn get_permissions_for_user(id: Uuid, pool: &PgPool) -> Result<Vec<Permission>> {
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
