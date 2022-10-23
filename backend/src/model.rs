use crate::schema::*;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Insertable, AsChangeset, Deserialize, Serialize, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Insertable, AsChangeset, Deserialize, Serialize, Queryable)]
#[table_name = "plants"]
pub struct Plant {
    pub id: i32,
    pub name: String,
    pub watering_freq: i32,
    pub uid: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
