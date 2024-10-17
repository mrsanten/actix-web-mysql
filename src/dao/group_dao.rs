use super::Group;
use super::Table;

impl Table<Group> {
    pub async fn get_group_by_id(&self, id: u64) -> Result<Group, sqlx::Error> {
        sqlx::query_as::<_, Group>(
            r#"
            SELECT `id`, `name`
            FROM `groups`
            WHERE `id` = ?
        "#,
        )
        .bind(id)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn get_group_by_name(&self, name: &str) -> Result<Group, sqlx::Error> {
        sqlx::query_as::<_, Group>(
            r#"
            SELECT `id`, `name`
            FROM `groups`
            WHERE `name` = ?
        "#,
        )
        .bind(name)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn add_group(&self, name: &str) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO `groups` (`name`)
            VALUES (?)
        "#,
        )
        .bind(name)
        .execute(&*self.pool)
        .await
        .map(|result| result.last_insert_id())
    }

    pub async fn update_group(&self, current: &str, update: &str) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE `groups`
            SET `name` = ?
            WHERE `name` = ?
        "#,
        )
        .bind(update)
        .bind(current)
        .execute(&*self.pool)
        .await
        .map(|result| result.rows_affected())
    }

    pub async fn delete_group(&self, name: &str) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM `groups`
            WHERE `name` = ?
        "#,
        )
        .bind(name)
        .execute(&*self.pool)
        .await
        .map(|result| result.rows_affected())
    }
}