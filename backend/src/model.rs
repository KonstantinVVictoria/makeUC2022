use crate::schema::*;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Insertable, AsChangeset, Deserialize, Serialize, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub uid: String,
    pub username: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub uid: String,
    pub username: String,
}

#[derive(Insertable, AsChangeset, Deserialize, Serialize, Queryable)]
#[table_name = "plants"]
pub struct Plant {
    pub id: i32,
    pub uid: String,
    pub symbol: String,
    pub sci_name: String,
    pub gener_name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "plants"]
pub struct NewPlant {
    pub uid: String,
    pub symbol: String,
    pub sci_name: String,
    pub gener_name: String,
}
