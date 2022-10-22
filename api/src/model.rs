use crate::schema::plants;
use crate::schema::users;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Insertable, AsChangeset, Queryable, Debug, Deserialize, Serialize)]
#[table_name = "plants"]
pub struct Plant {
    pub id: i32,
    pub name: String,
    pub water_frequency: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Queryable, Debug, Deserialize, Serialize)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}
