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
pub async fn create_plant(conn: crate::DbConn, plant: Json<NewPlant>) -> Json<Plant> {
    conn.run(move |c| {
        diesel::insert_into(plants::table)
            .values(plant.into_inner())
            .get_result(c)
    })
    .await
    .map(Json)
    .expect("Failed to POST plant")
}

/// GET
/// Read a plant by ID belonging to a user
/// mount point:
///  /api/plant/<uid>/<id>
#[get("/<uid>/<id>")]
pub async fn read_plant(conn: crate::DbConn, uid: String, id: i32) -> Json<Plant> {
    conn.run(move |c| {
        plants::table
            .filter(plants::uid.eq(uid))
            .filter(plants::id.eq(id))
            .first(c)
    })
    .await
    .map(Json)
    .expect("Failed to GET plant")
}

/// GET
/// Read all plants belonging to a uid
/// mount point:
///     /api/plants/<uid>
#[get("/<uid>")]
pub async fn read_all_plants(conn: crate::DbConn, uid: String) -> Json<Vec<Plant>> {
    conn.run(move |c| plants::table.filter(plants::uid.eq(uid)).load::<Plant>(c))
        .await
        .map(Json)
        .expect("Failed to get plants by uid")
}

/// UPDATE plant
/// Update a plant by ID belonging to a user
/// mount point:
///     /api/plants/<uid>/<id>
#[post("/<uid>/<id>", data = "<plant>")]
pub async fn update_plant(
    conn: crate::DbConn,
    uid: String,
    id: i32,
    plant: Json<Plant>,
) -> Json<Plant> {
    conn.run(move |c| {
        diesel::update(plants::table)
            .filter(plants::uid.eq(uid))
            .filter(plants::id.eq(id))
            .set(plant.into_inner())
            .get_result(c)
    })
    .await
    .map(Json)
    .expect("Failed to update plant")
}

/// DELETE plant
/// Delete a plant by ID belonging to a user
/// mount point:
///    /api/plants/<uid>/<id>
#[delete("/<uid>/<id>")]
pub async fn delete_plant(conn: crate::DbConn, uid: String, id: i32) -> Json<Plant> {
    conn.run(move |c| {
        diesel::delete(plants::table)
            .filter(plants::uid.eq(uid))
            .filter(plants::id.eq(id))
            .get_result(c)
    })
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
pub async fn create_user(conn: crate::DbConn, user: Json<NewUser>) -> Json<User> {
    conn.run(move |c| {
        diesel::insert_into(users::table)
            .values(user.into_inner())
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
#[get("/<uid>")]
pub async fn read_user(conn: crate::DbConn, uid: String) -> Json<User> {
    conn.run(move |c| users::table.filter(users::uid.eq(uid)).first(c))
        .await
        .map(Json)
        .expect("Failed to GET user")
}

/// UPDATE
/// Update a user by ID
/// mount point:
///   /api/user/<id>
#[post("/<uid>", data = "<user>")]
pub async fn update_user(conn: crate::DbConn, uid: String, user: Json<User>) -> Json<User> {
    conn.run(move |c| {
        diesel::update(users::table)
            .filter(users::uid.eq(uid))
            .set(user.into_inner())
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
#[delete("/<uid>")]
pub async fn delete_user(conn: crate::DbConn, uid: String) -> Json<User> {
    conn.run(move |c| {
        diesel::delete(users::table)
            .filter(users::uid.eq(uid))
            .get_result(c)
    })
    .await
    .map(Json)
    .expect("Failed to delete user")
}
