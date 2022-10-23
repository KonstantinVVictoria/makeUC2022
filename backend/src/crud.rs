use crate::model::*;
use crate::schema::*;
use diesel::{QueryDsl, RunQueryDsl};
use rocket::serde::json::Json;

/// POST
/// Create a new plant
/// mount point:
///   /api/create
#[post("/", data = "<plant>")]
pub async fn create_plant(conn: crate::DbConn, plant: Json<Plant>) -> Json<Plant> {
    conn.run(move |c| {
        diesel::insert_into(plants::table)
            .values(&plant.into_inner())
            .get_result(c)
    })
    .await
    .map(Json)
    .expect("Failed to POST plant")
}

/// GET
/// Read a plant by ID
#[get("/<id>")]
pub async fn read_plant(conn: crate::DbConn, id: i32) -> Json<Plant> {
    conn.run(move |c| plants::table.find(id).get_result(c))
        .await
        .map(Json)
        .expect("Failed to get plant by id")
}

/// UPDATE plant
#[post("/<id>", data = "<plant>")]
pub async fn update_plant(conn: crate::DbConn, id: i32, plant: Json<Plant>) -> Json<Plant> {
    conn.run(move |c| {
        diesel::update(plants::table.find(id))
            .set(&plant.into_inner())
            .get_result(c)
    })
    .await
    .map(Json)
    .expect("Failed to update plant")
}

/// DELETE plant
#[delete("/<id>")]
pub async fn delete_plant(conn: crate::DbConn, id: i32) -> Json<Plant> {
    conn.run(move |c| diesel::delete(plants::table.find(id)).get_result(c))
        .await
        .map(Json)
        .expect("Failed to delete plant")
}
