use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, Row};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Group {
    pub id: u64,
    pub name: String,
}

impl<'c> FromRow<'c, MySqlRow> for Group {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Group {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
        })
    }
}