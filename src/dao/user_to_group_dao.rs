use super::{Group, JoinTable, User};

impl JoinTable<User, Group> {
    pub async fn add_user_groups(
        &self,
        user_id: &str,
        groups: &[Group],
    ) -> Result<u64, sqlx::Error> {
        if groups.is_empty() {
            Ok(0)
        } else {
            let insert_statement = build_insert_statement(groups.len());
            let mut query = sqlx::query(&insert_statement);

            for group in groups {
                query = query.bind(user_id).bind(group.id);
            }

            query.execute(&*self.pool)
                .await
                .map(|result| result.rows_affected())
        }
    }

    pub async fn get_groups_by_user_id(&self, user_id: &str) -> Result<Vec<Group>, sqlx::Error> {
        sqlx::query_as::<_, Group>(
            r#"
            SELECT * FROM `groups` AS `a`
            WHERE `a`.`id` IN (
                SELECT `b`.`group_id` FROM `users_to_groups` AS `b`
                WHERE `b`.`user_id` = ?
            )
        "#,
        )
        .bind(user_id)
        .fetch_all(&*self.pool)
        .await
    }

    pub async fn delete_by_user_id(&self, user_id: &str) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            DELETE
            FROM `users_to_groups`
            WHERE `user_id` = ?
        "#,
        )
        .bind(user_id)
        .execute(&*self.pool)
        .await
        .map(|result| result.rows_affected())
    }

    pub async fn delete_by_group_id(&self, group_id: u64) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            DELETE
            FROM `users_to_groups`
            WHERE `group_id` = ?
        "#,
        )
        .bind(group_id)
        .execute(&*self.pool)
        .await
        .map(|result| result.rows_affected())
    }

    pub async fn update_user_groups(&self, user: &User) -> Result<u64, sqlx::Error> {
        if user.groups.is_empty() {
            self.delete_by_user_id(&user.id).await
        } else {
            let deleted = self.delete_by_user_id(&user.id).await?;
            let added = self.add_user_groups(&user.id, &user.groups).await?;
            Ok(added + deleted)
        }
    }
}

static DEFAULT_INSERT: &str = r#"
    INSERT INTO `users_to_groups` (`user_id`, `group_id`)
    VALUES (?,?)
"#;

fn build_insert_statement(rows: usize) -> String {
    let mut insert = String::from(DEFAULT_INSERT);

    match rows {
        1 | 0 => insert,
        _ => {
            let mut i = 1;
            while i < rows {
                insert.push_str(", (?,?)");
                i += 1;
            }
            insert
        }
    }
}

#[cfg(test)]
mod test {
    use super::{build_insert_statement, DEFAULT_INSERT};

    #[test]
    fn build_insert_statement_returns_default_string_when_input_is_zero_or_one() {
        let results = vec![build_insert_statement(0), build_insert_statement(1)];

        assert_eq!(results[0], results[1]);
        assert_eq!(results[0], DEFAULT_INSERT);
    }

    #[test]
    fn build_insert_statement_returns_n_parameters_when_input_is_n() {
        let result = build_insert_statement(3);

        assert_eq!(
            format!("{0}{1}{2}", DEFAULT_INSERT, ", (?,?)", ", (?,?)"),
            result
        );
    }
}