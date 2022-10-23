use crate::diesel::ExpressionMethods;
use crate::model::*;
use crate::schema::*;
use diesel::{QueryDsl, RunQueryDsl};
use rocket::serde::json::Json;

// ----------------------------------------------------------------
// PLANT
// ----------------------------------------------------------------

/// POST
/// Create a new plant
/// mount point:
///   /api/plants/<plant>
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
/// mount point:
///  /api/plants/<id>
#[get("/<id>")]
pub async fn read_plant(conn: crate::DbConn, id: i32) -> Json<Plant> {
    conn.run(move |c| plants::table.find(id).get_result(c))
        .await
        .map(Json)
        .expect("Failed to get plant by id")
}

/// GET
/// Read all plants belonging to a uid
/// mount point:
///     /api/plants/<uid>
#[get("/<uid>")]
pub async fn read_all_plants(conn: crate::DbConn, uid: i32) -> Json<Vec<Plant>> {
    conn.run(move |c| plants::table.filter(plants::uid.eq(uid)).load::<Plant>(c))
        .await
        .map(Json)
        .expect("Failed to get plants by uid")
}

/// UPDATE plant
/// Update a plant by ID
/// mount point:
///     /api/plants/<id>
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
/// Delete a plant by ID
/// mount point:
///    /api/plants/<id>
#[delete("/<id>")]
pub async fn delete_plant(conn: crate::DbConn, id: i32) -> Json<Plant> {
    conn.run(move |c| diesel::delete(plants::table.find(id)).get_result(c))
        .await
        .map(Json)
        .expect("Failed to delete plant")
}

// ----------------------------------------------------------------
// USER
// ----------------------------------------------------------------

/// POST
/// Create a new user
/// mount point:
///     /api/user/create
#[post("/", data = "<user>")]
pub async fn create_user(conn: crate::DbConn, user: Json<User>) -> Json<User> {
    conn.run(move |c| {
        diesel::insert_into(users::table)
            .values(&user.into_inner())
            .get_result(c)
    })
    .await
    .map(Json)
    .expect("Failed to POST user")
}

/// GET
/// Read a user by ID
/// mount point:
///    /api/user/<id>
#[get("/<id>")]
pub async fn read_user(conn: crate::DbConn, id: i32) -> Json<User> {
    conn.run(move |c| users::table.find(id).get_result(c))
        .await
        .map(Json)
        .expect("Failed to get user by id")
}

/// UPDATE
/// Update a user by ID
/// mount point:
///   /api/user/<id>
#[post("/<id>", data = "<user>")]
pub async fn update_user(conn: crate::DbConn, id: i32, user: Json<User>) -> Json<User> {
    conn.run(move |c| {
        diesel::update(users::table.find(id))
            .set(&user.into_inner())
            .get_result(c)
    })
    .await
    .map(Json)
    .expect("Failed to update user")
}

/// DELETE
/// Delete a user by ID
/// mount point:
///  /api/user/<id>
#[delete("/<id>")]
pub async fn delete_user(conn: crate::DbConn, id: i32) -> Json<User> {
    conn.run(move |c| diesel::delete(users::table.find(id)).get_result(c))
        .await
        .map(Json)
        .expect("Failed to delete user")
}
