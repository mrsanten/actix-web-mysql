use super::{Group, User};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, MySqlPool};
use std::sync::Arc;

pub struct Table<T>
where
    T: for<'c> FromRow<'c, MySqlRow>,
{
    pub pool: Arc<MySqlPool>,
    _from_row: for<'c> fn(&'c MySqlRow) -> Result<T, sqlx::Error>,
}

impl<T> Table<T>
where
    T: for<'c> FromRow<'c, MySqlRow>,
{
    fn new(pool: Arc<MySqlPool>) -> Self {
        // Wrap T::from_row in a closure to match the expected function pointer type
        Table {
            pool,
            _from_row: |row: &MySqlRow| T::from_row(row),
        }
    }
}

pub struct JoinTable<T1, T2>
where
    T1: for<'c> FromRow<'c, MySqlRow>,
    T2: for<'c> FromRow<'c, MySqlRow>,
{
    pub pool: Arc<MySqlPool>,
    _from_row: (
        for<'c> fn(&'c MySqlRow) -> Result<T1, sqlx::Error>,
        for<'c> fn(&'c MySqlRow) -> Result<T2, sqlx::Error>,
    ),
}

impl<T1, T2> JoinTable<T1, T2>
where
    T1: for<'c> FromRow<'c, MySqlRow>,
    T2: for<'c> FromRow<'c, MySqlRow>,
{
    fn new(pool: Arc<MySqlPool>) -> Self {
        // Wrap T1::from_row and T2::from_row in closures to match the expected function pointer types
        JoinTable {
            pool,
            _from_row: (
                |row: &MySqlRow| T1::from_row(row),
                |row: &MySqlRow| T2::from_row(row),
            ),
        }
    }
}

pub struct Database {
    pub groups: Arc<Table<Group>>,
    pub users: Arc<Table<User>>,
    pub users_to_groups: Arc<JoinTable<User, Group>>,
}

impl Database {
    pub async fn new(sql_url: &str) -> Result<Self, sqlx::Error> {
        let connection = MySqlPool::connect(sql_url).await?;
        let pool = Arc::new(connection);

        Ok(Database {
            groups: Arc::new(Table::new(pool.clone())),
            users: Arc::new(Table::new(pool.clone())),
            users_to_groups: Arc::new(JoinTable::new(pool.clone())),
        })
    }
}