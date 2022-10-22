use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Plant {
    pub id: i32,
    pub name: String,
    pub watering_freq: i32,
    pub created_at: chrono::NaiveDateTime,
}